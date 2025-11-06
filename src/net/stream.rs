use crate::common::*;
use std::{io, net::Shutdown, path::Path};

use windows::Win32::Networking::WinSock::{
    self, AF_UNIX, INVALID_SOCKET, SEND_RECV_FLAGS, SOCK_STREAM, SOCKADDR_UN, SOCKET, SOCKET_ERROR,
    WSA_FLAG_OVERLAPPED,
};

pub struct UnixStream(SOCKET);
impl UnixStream {
    pub fn connect<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        unsafe {
            startup()?;
            let mut s = WinSock::socket(AF_UNIX as _, SOCK_STREAM, WSA_FLAG_OVERLAPPED as _)?;
            if s == INVALID_SOCKET {
                return Err(wsa_error());
            }
            let addr = socketaddr_un(path)?;
            let err = WinSock::connect(
                s,
                &addr as *const _ as *const _,
                size_of::<SOCKADDR_UN>() as _,
            );
            if err == SOCKET_ERROR {
                WinSock::closesocket(s);
                s = INVALID_SOCKET;
            }
            Ok(Self(s))
        }
    }
    pub fn shutdown(&mut self, how: Shutdown) -> io::Result<()> {
        use WinSock::{SD_BOTH, SD_RECEIVE, SD_SEND};
        let shutdown_how = match how {
            Shutdown::Read => SD_RECEIVE,
            Shutdown::Write => SD_SEND,
            Shutdown::Both => SD_BOTH,
        };
        unsafe {
            if WinSock::shutdown(self.0, shutdown_how) != 0 {
                Err(wsa_error())
            } else {
                Ok(())
            }
        }
    }
}

impl io::Write for UnixStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unsafe {
            match WinSock::send(self.0, buf, SEND_RECV_FLAGS(0)) {
                SOCKET_ERROR => Err(wsa_error()),
                value => Ok(value as _),
            }
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        todo!()
    }
}

impl io::Read for UnixStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        unsafe {
            match WinSock::recv(self.0, buf, SEND_RECV_FLAGS(0)) {
                0 => Err(io::Error::other("Connection closed")),
                value if value > 0 => Ok(value as _),
                _ => Err(wsa_error()),
            }
        }
    }
}
