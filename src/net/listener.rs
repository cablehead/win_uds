use crate::net::{SockAddr, Socket, UnixStream};
use socket2::{Domain, Type};
use std::{
    io,
    ops::{Deref, DerefMut},
    os::windows::io::{AsRawSocket, AsSocket, IntoRawSocket},
    path::Path,
};
pub struct UnixListener(pub Socket);

impl UnixListener {
    pub fn bind<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let addr = SockAddr::unix(path)?;
        Self::bind_addr(&addr)
    }
    pub fn bind_addr(socket_addr: &SockAddr) -> io::Result<Self> {
        let s = Socket::new(Domain::UNIX, Type::STREAM, None)?;
        s.bind(socket_addr)?;
        s.listen(5)?;
        Ok(Self(s))
    }
    pub fn accept(&self) -> io::Result<(UnixStream, SockAddr)> {
        let (s, addr) = self.0.accept()?;
        Ok((UnixStream(s), addr))
    }
}
impl AsSocket for UnixListener {
    fn as_socket(&self) -> std::os::windows::prelude::BorrowedSocket<'_> {
        self.0.as_socket()
    }
}
impl AsRawSocket for UnixListener {
    fn as_raw_socket(&self) -> std::os::windows::prelude::RawSocket {
        self.0.as_raw_socket()
    }
}
impl IntoRawSocket for UnixListener {
    fn into_raw_socket(self) -> std::os::windows::prelude::RawSocket {
        self.0.into_raw_socket()
    }
}
impl Deref for UnixListener {
    type Target = Socket;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for UnixListener {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
