#![cfg(not(target_os = "wasi"))]
use std::io;
use std::fmt;
use std::net::{self, SocketAddr, Ipv4Addr, Ipv6Addr};
use std::sync::Arc;
use crate::sys::AsRawSocketArc;
use std::os::arceos::net::AxUdpSocketHandle;
use std::os::arceos::net::AxSocketHandle;
use crate::sys::arceos::AxSocketHandle::Udp;
use std::os::arceos::net::IntoRawUdpSocket;
use std::os::arceos::api::net as api;
use super::cvt;

pub fn bind(addr: SocketAddr) -> io::Result<AxUdpSocketHandle> {
    cvt(api::ax_udp_bind(addr))
}

pub(crate) fn only_v6(_: &AxUdpSocket) -> io::Result<bool> {
    os_required!()
}

pub(crate) struct AxUdpSocket {
    inner: Arc<AxSocketHandle>,
}

impl AxUdpSocket {
    pub fn from_std(socket: net::UdpSocket) -> Self {
        Self {
            inner: Arc::new(Udp(socket.into_raw_socket())),
        }
    }

    pub unsafe fn from_raw_socket(handle: AxSocketHandle) -> Self {
        Self {
            inner: Arc::new(handle),
        }
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        cvt(api::ax_udp_socket_addr(&self.as_raw_socket()))
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        os_required!()
    }

    pub fn send(&self, buf: &[u8]) -> io::Result<usize> {
        cvt(api::ax_udp_send(&self.as_raw_socket(), buf))
    }

    pub fn recv(&self, _buf: &mut [u8]) -> io::Result<usize> {
        os_required!()
    }

    pub fn peek(&self, _buf: &mut [u8]) -> io::Result<usize> {
        os_required!()
    }

    pub fn peek_from(&self, _buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        os_required!()
    }

    pub fn send_to(&self, buf: &[u8], target: SocketAddr) -> io::Result<usize> {
        cvt(api::ax_udp_send_to(&self.as_raw_socket(), buf, target))
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        cvt(api::ax_udp_recv_from(&self.as_raw_socket(), buf))
    }

    pub fn connect(&self, addr: SocketAddr) -> io::Result<()> {
        cvt(api::ax_udp_connect(&self.as_raw_socket(), addr))
    }

    pub fn set_broadcast(&self, _on: bool) -> io::Result<()> {
        os_required!()
    }

    pub fn broadcast(&self) -> io::Result<bool> {
        os_required!()
    }

    pub fn set_multicast_ttl_v4(&self, _ttl: u32) -> io::Result<()> {
        os_required!()
    }

    pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
        os_required!()
    }

    pub fn set_multicast_loop_v4(&self, _on: bool) -> io::Result<()> {
        os_required!()
    }

    pub fn multicast_loop_v4(&self) -> io::Result<bool> {
        os_required!()
    }

    pub fn set_multicast_loop_v6(&self, _on: bool) -> io::Result<()> {
        os_required!()
    }

    pub fn multicast_loop_v6(&self) -> io::Result<bool> {
        os_required!()
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        os_required!()
    }

    pub fn ttl(&self) -> io::Result<u32> {
        os_required!()
    }

    pub fn join_multicast_v4(&self, _multiaddr: &Ipv4Addr, _interface: &Ipv4Addr) -> io::Result<()> {
        os_required!()
    }

    pub fn join_multicast_v6(&self, _multiaddr: &Ipv6Addr, _interface: u32) -> io::Result<()> {
        os_required!()
    }

    pub fn leave_multicast_v4(&self, _multiaddr: &Ipv4Addr, _interface: &Ipv4Addr) -> io::Result<()> {
        os_required!()
    }

    pub fn leave_multicast_v6(&self, _multiaddr: &Ipv6Addr, _interface: u32) -> io::Result<()> {
        os_required!()
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        Ok(None)
    }

    pub fn as_raw_socket(&self) -> &AxUdpSocketHandle {
        if let Udp(handle) = &self.inner.as_ref() {
            handle
        } else {
            panic!("NOT udp handle!");
        }
    }
}

impl AsRawSocketArc for AxUdpSocket {
    #[inline]
    fn as_raw_socket_arc(&self) -> &Arc<AxSocketHandle> {
        &self.inner
    }
}

impl fmt::Debug for AxUdpSocket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AxUdpSocket")
    }
}
