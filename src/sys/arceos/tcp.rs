use std::fmt;
use std::io::{self, IoSlice, IoSliceMut, Read, Write};
use std::net::{self, Shutdown, SocketAddr};
use std::sync::Arc;

use std::os::arceos::api::net as api;
use std::os::arceos::net::{AxTcpSocketHandle, IntoRawTcpSocket};

use super::cvt;

pub(crate) fn new_for_addr(_: SocketAddr) -> io::Result<AxTcpSocketHandle> {
    let socket = api::ax_tcp_socket();
    cvt(api::ax_tcp_set_nonblocking(&socket, true))?;
    Ok(socket)
}

pub(crate) fn bind(socket: &AxTcpListener, addr: SocketAddr) -> io::Result<()> {
    cvt(api::ax_tcp_bind(&socket.inner, addr))
}

pub(crate) fn connect(socket: &AxTcpStream, addr: SocketAddr) -> io::Result<()> {
    cvt(api::ax_tcp_connect(&socket.inner, addr))
}

pub(crate) fn listen(socket: &AxTcpListener, backlog: u32) -> io::Result<()> {
    cvt(api::ax_tcp_listen(&socket.inner, backlog as usize))
}

pub(crate) fn accept(socket: &AxTcpListener) -> io::Result<(AxTcpStream, SocketAddr)> {
    let (handle, addr) = cvt(api::ax_tcp_accept(&socket.inner))?;
    cvt(api::ax_tcp_set_nonblocking(&handle, true))?;
    let socket = unsafe { AxTcpStream::from_raw_socket(handle) };
    Ok((socket, addr))
}

pub(crate) trait AsRawTcpSocketArc {
    fn as_raw_socket_arc(&self) -> &Arc<AxTcpSocketHandle>;
}

////////////////////////////////////////////////////////////////////////////////
// TCP streams
////////////////////////////////////////////////////////////////////////////////

pub(crate) struct AxTcpStream {
    inner: Arc<AxTcpSocketHandle>,
}

impl AxTcpStream {
    pub fn from_std(stream: net::TcpStream) -> Self {
        Self {
            inner: Arc::new(stream.into_raw_socket()),
        }
    }

    pub unsafe fn from_raw_socket(handle: AxTcpSocketHandle) -> Self {
        Self {
            inner: Arc::new(handle),
        }
    }

    pub fn into_raw_socket(self) -> AxTcpSocketHandle {
        Arc::into_inner(self.inner).unwrap()
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        cvt(api::ax_tcp_peer_addr(&self.inner))
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        cvt(api::ax_tcp_socket_addr(&self.inner))
    }

    pub fn shutdown(&self, _: Shutdown) -> io::Result<()> {
        cvt(api::ax_tcp_shutdown(&self.inner))
    }

    pub fn set_nodelay(&self, _: bool) -> io::Result<()> {
        os_required!()
    }

    pub fn nodelay(&self) -> io::Result<bool> {
        os_required!()
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        os_required!()
    }

    pub fn ttl(&self) -> io::Result<u32> {
        os_required!()
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        os_required!()
    }

    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        os_required!()
    }

    pub fn _read(&self, buf: &mut [u8]) -> io::Result<usize> {
        cvt(api::ax_tcp_recv(&self.inner, buf))
    }

    pub fn _read_vectored(&self, _: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        os_required!()
    }

    pub fn _write(&self, buf: &[u8]) -> io::Result<usize> {
        cvt(api::ax_tcp_send(&self.inner, buf))
    }

    pub fn _write_vectored(&self, _: &[IoSlice<'_>]) -> io::Result<usize> {
        os_required!()
    }
}

impl AsRawTcpSocketArc for AxTcpStream {
    #[inline]
    fn as_raw_socket_arc(&self) -> &Arc<AxTcpSocketHandle> {
        &self.inner
    }
}

impl fmt::Debug for AxTcpStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = f.debug_struct("AxTcpStream");

        if let Ok(addr) = self.local_addr() {
            res.field("addr", &addr);
        }

        if let Ok(peer) = self.peer_addr() {
            res.field("peer", &peer);
        }

        res.finish()
    }
}

impl Read for &AxTcpStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self._read(buf)
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        self._read_vectored(bufs)
    }
}

impl Write for &AxTcpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self._write(buf)
    }

    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        self._write_vectored(bufs)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////
// TCP listeners
////////////////////////////////////////////////////////////////////////////////

pub(crate) struct AxTcpListener {
    inner: Arc<AxTcpSocketHandle>,
}

impl AxTcpListener {
    pub fn from_std(listener: net::TcpListener) -> Self {
        Self {
            inner: Arc::new(listener.into_raw_socket()),
        }
    }

    pub unsafe fn from_raw_socket(handle: AxTcpSocketHandle) -> Self {
        Self {
            inner: Arc::new(handle),
        }
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        cvt(api::ax_tcp_socket_addr(&self.inner))
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        os_required!()
    }

    pub fn ttl(&self) -> io::Result<u32> {
        os_required!()
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        os_required!()
    }
}

impl AsRawTcpSocketArc for AxTcpListener {
    #[inline]
    fn as_raw_socket_arc(&self) -> &Arc<AxTcpSocketHandle> {
        &self.inner
    }
}

impl fmt::Debug for AxTcpListener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = f.debug_struct("AxTcpListener");

        if let Ok(addr) = self.local_addr() {
            res.field("addr", &addr);
        }

        res.finish()
    }
}
