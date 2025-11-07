use crate::net::{UnixListener, UnixStream};
use futures::{AsyncRead, AsyncWrite};
use socket2::SockAddr;
use std::{
    io::{self, Read, Write},
    ops::{Deref, DerefMut},
    path::Path,
};

pub struct AsyncStream(UnixStream);
impl AsyncStream {
    pub fn connect<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let s = UnixStream::connect(path)?;
        s.set_nonblocking(true)?;
        Ok(Self(s))
    }
}
impl AsyncRead for AsyncStream {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> std::task::Poll<io::Result<usize>> {
        match self.0.read(buf) {
            Ok(n) => std::task::Poll::Ready(Ok(n)),
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                cx.waker().wake_by_ref();
                std::task::Poll::Pending
            }
            Err(e) => std::task::Poll::Ready(Err(e)),
        }
    }
}
impl AsyncWrite for AsyncStream {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<io::Result<usize>> {
        match self.0.write(buf) {
            Ok(n) => std::task::Poll::Ready(Ok(n)),
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                cx.waker().wake_by_ref();
                std::task::Poll::Pending
            }
            Err(e) => std::task::Poll::Ready(Err(e)),
        }
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), io::Error>> {
        match self.0.shutdown(std::net::Shutdown::Write) {
            Ok(()) => std::task::Poll::Ready(Ok(())),
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                cx.waker().wake_by_ref();
                std::task::Poll::Pending
            }
            Err(e) => std::task::Poll::Ready(Err(e)),
        }
    }
}
pub struct AsyncListener(UnixListener);
impl AsyncListener {
    pub fn accept(&self) -> io::Result<(AsyncStream, SockAddr)> {
        let (s, addr) = self.0.accept()?;
        Ok((AsyncStream(s), addr))
    }
    pub fn bind<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let s = UnixListener::bind(path)?;
        s.set_nonblocking(true)?;
        Ok(Self(s))
    }
}
impl Deref for AsyncStream {
    type Target = UnixStream;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for AsyncStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for AsyncListener {
    type Target = UnixListener;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AsyncListener {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
