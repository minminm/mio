use crate::{Interest, Token};

use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex, Weak};
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicUsize, Ordering};

use std::os::arceos::api::net::{self as api, AxSocketHandle};

#[derive(Clone)]
pub struct Event {
    pub token: Token,
    pub is_readable: bool,
    pub is_writable: bool,
    pub is_error: bool,
}

pub type Events = Vec<Event>;

impl Event {
    fn new(token: usize, is_readable: bool, is_writable: bool) -> Self {
        Self {
            token: Token(token),
            is_readable,
            is_writable,
            is_error: false,
        }
    }

    fn new_error(token: usize) -> Self {
        Self {
            token: Token(token),
            is_readable: false,
            is_writable: false,
            is_error: true,
        }
    }
}

#[derive(Debug)]
struct SocketState {
    inner: Weak<AxSocketHandle>,
    interests: Interest,
}

#[derive(Debug)]
pub struct Selector {
    sockets: Arc<Mutex<HashMap<usize, SocketState>>>,
    waiting: AtomicUsize,
}

impl Selector {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            sockets: Arc::new(Mutex::new(HashMap::new())),
            waiting: AtomicUsize::new(0),
        })
    }

    pub fn try_clone(&self) -> io::Result<Selector> {
        Ok(Self {
            sockets: self.sockets.clone(),
            waiting: AtomicUsize::new(0),
        })
    }

    pub fn select(&self, events: &mut Events, timeout: Option<Duration>) -> io::Result<()> {
        let deadline = timeout.map(|t| Instant::now() + t);

        let max_events = events.capacity();
        events.clear();
        loop {
            super::cvt(api::ax_poll_interfaces())?;
            self.sockets
                .lock()
                .unwrap()
                .retain(|&token, SocketState { inner, interests }| {
                    if let Some(socket) = inner.upgrade() {
                        if events.len() < max_events {
                            match socket.as_ref() {
                                AxSocketHandle::Tcp(socket) => {
                                    match api::ax_tcp_poll(&socket) {
                                        Ok(res) => {
                                            let event = Event::new(
                                                token,
                                                interests.is_readable() && res.readable,
                                                interests.is_writable() && res.writable,
                                            );
                                            if event.is_readable || event.is_writable {
                                                events.push(event);
                                            }
                                        }
                                        Err(_) => events.push(Event::new_error(token)),
                                    }
                                },
                                AxSocketHandle::Udp(socket) => {
                                    match api::ax_udp_poll(&socket) {
                                        Ok(res) => {
                                            let event = Event::new(
                                                token,
                                                interests.is_readable() && res.readable,
                                                interests.is_writable() && res.writable,
                                            );
                                            //println!("+++++++++++++ event {:?} {:?}", event.is_readable, event.is_writable);
                                            if event.is_readable || event.is_writable {
                                                events.push(event);
                                            }
                                        }
                                        Err(_) => events.push(Event::new_error(token)),
                                    }
                                }
                            }
                        }
                        true
                    } else {
                        false
                    }
                });

            if !events.is_empty() {
                const MAX_WAITING_TIMES: usize = 9;
                if self.waiting.fetch_add(1, Ordering::SeqCst) > MAX_WAITING_TIMES {
                    self.waiting.store(0, Ordering::Relaxed);
                    std::thread::yield_now();
                }
                return Ok(());
            }
            if deadline.map_or(false, |ddl| Instant::now() >= ddl) {
                return Err(io::ErrorKind::Interrupted.into());
            }
            std::thread::yield_now();
            self.waiting.fetch_add(1, Ordering::SeqCst);
        }
    }

    pub fn register(
        &self,
        token: Token,
        interests: Interest,
        socket: &Arc<AxSocketHandle>,
    ) -> io::Result<()> {
        self.sockets.lock().unwrap().insert(
            usize::from(token),
            SocketState {
                inner: Arc::downgrade(socket),
                interests,
            },
        );
        Ok(())
    }

    pub fn reregister(
        &self,
        token: Token,
        interests: Interest,
        socket: &Arc<AxSocketHandle>,
    ) -> io::Result<()> {
        let mut sockets = self.sockets.lock().unwrap();
        let entry = sockets
            .get_mut(&usize::from(token))
            .ok_or(io::Error::from(io::ErrorKind::InvalidInput))?;
        if let Some(sock) = entry.inner.upgrade() {
            if Arc::ptr_eq(&sock, socket) {
                entry.interests = interests;
                return Ok(());
            }
        }
        Err(io::ErrorKind::InvalidInput.into())
    }

    pub fn deregister(&self, _socket: &Arc<AxSocketHandle>) -> io::Result<()> {
        // deregistration is done in `select()` after the last strong reference is dropped
        Ok(())
    }

    #[cfg(all(debug_assertions, not(target_os = "wasi")))]
    pub fn register_waker(&self) -> bool {
        os_required!();
    }
}

cfg_io_source! {
    #[cfg(debug_assertions)]
    impl Selector {
        pub fn id(&self) -> usize {
            os_required!();
        }
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
pub mod event {
    use crate::sys::Event;
    use crate::Token;
    use std::fmt;

    pub fn token(event: &Event) -> Token {
        event.token
    }

    pub fn is_readable(event: &Event) -> bool {
        event.is_readable
    }

    pub fn is_writable(event: &Event) -> bool {
        event.is_writable
    }

    pub fn is_error(event: &Event) -> bool {
        event.is_error
    }

    pub fn is_read_closed(_: &Event) -> bool {
        false
    }

    pub fn is_write_closed(_: &Event) -> bool {
        false
    }

    pub fn is_priority(_: &Event) -> bool {
        false
    }

    pub fn is_aio(_: &Event) -> bool {
        false
    }

    pub fn is_lio(_: &Event) -> bool {
        false
    }

    pub fn debug_details(_: &mut fmt::Formatter<'_>, _: &Event) -> fmt::Result {
        os_required!();
    }
}
