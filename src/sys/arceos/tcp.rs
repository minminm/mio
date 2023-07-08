use std::io;
use std::net::{self, SocketAddr};

use std::os::arceos::net::{AsRawTcpSocket, AxTcpSocketHandle, FromRawTcpSocket};
use std::os::arceos::api::net as api;

use super::cvt;

pub(crate) fn new_for_addr(_: SocketAddr) -> io::Result<AxTcpSocketHandle> {
    Ok(api::ax_tcp_socket())
}

pub(crate) fn bind(socket: &net::TcpListener, addr: SocketAddr) -> io::Result<()> {
    cvt(api::ax_tcp_bind(socket.as_raw_socket(), addr))
}

pub(crate) fn connect(socket: &net::TcpStream, addr: SocketAddr) -> io::Result<()> {
    cvt(api::ax_tcp_connect(socket.as_raw_socket(), addr))
}

pub(crate) fn listen(socket: &net::TcpListener, backlog: u32) -> io::Result<()> {
    cvt(api::ax_tcp_listen(socket.as_raw_socket(), backlog as usize))
}

pub(crate) fn accept(socket: &net::TcpListener) -> io::Result<(net::TcpStream, SocketAddr)> {
    let (handle, addr) = cvt(api::ax_tcp_accept(socket.as_raw_socket()))?;
    let socket = unsafe { net::TcpStream::from_raw_socket(handle) };
    // socket.set_nonblocking(true)?;
    Ok((socket, addr))
}
