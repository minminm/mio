macro_rules! os_required {
    () => {
        panic!("unimplemented OS operations in ArceOS")
    };
}

mod selector;
pub(crate) use self::selector::{event, Event, Events, Selector};

#[cfg(not(target_os = "wasi"))]
mod waker;
#[cfg(not(target_os = "wasi"))]
pub(crate) use self::waker::Waker;

use std::sync::Arc;
use std::os::arceos::api::AxResult;
use std::os::arceos::net::AxSocketHandle;

#[inline]
fn cvt<T>(t: AxResult<T>) -> std::io::Result<T> {
    match t {
        Ok(t) => Ok(t),
        Err(e) => Err(std::io::Error::from_raw_os_error(e.code())),
    }
}

cfg_net! {
    pub(crate) trait AsRawSocketArc {
        fn as_raw_socket_arc(&self) -> &Arc<AxSocketHandle>;
    }

    pub(crate) mod tcp;
    pub(crate) mod udp;
    #[cfg(unix)]
    pub(crate) mod uds;
}

cfg_io_source! {
    use std::io;
    #[cfg(windows)]
    use std::os::windows::io::RawSocket;

    #[cfg(windows)]
    use crate::{Registry, Token, Interest};

    pub(crate) struct IoSourceState;

    impl IoSourceState {
        pub fn new() -> IoSourceState {
            IoSourceState
        }

        pub fn do_io<T, F, R>(&self, f: F, io: &T) -> io::Result<R>
        where
            F: FnOnce(&T) -> io::Result<R>,
        {
            // We don't hold state, so we can just call the function and
            // return.
            f(io)
        }
    }

    #[cfg(windows)]
    impl IoSourceState {
         pub fn register(
            &mut self,
            _: &Registry,
            _: Token,
            _: Interest,
            _: RawSocket,
        ) -> io::Result<()> {
            os_required!()
        }

        pub fn reregister(
            &mut self,
            _: &Registry,
            _: Token,
            _: Interest,
        ) -> io::Result<()> {
           os_required!()
        }

        pub fn deregister(&mut self) -> io::Result<()> {
            os_required!()
        }
    }
}
