use std::{io, path::Path};

use windows::Win32::Networking::WinSock::{
    self, AF_UNIX, INVALID_SOCKET, SOCK_STREAM, SOCKADDR_UN, SOCKET, SOCKET_ERROR,
    WSA_FLAG_OVERLAPPED,
};

use crate::common::{socketaddr_un, startup, wsa_error};

pub struct UnixListener(SOCKET);

impl UnixListener {
    pub fn bind<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        unsafe {
            startup()?;
            let s = WinSock::socket(AF_UNIX as _, SOCK_STREAM, WSA_FLAG_OVERLAPPED as _)?;
            if s == INVALID_SOCKET {
                return Err(wsa_error());
            }
            let addr = socketaddr_un(path)?;
            let err = WinSock::bind(
                s,
                &addr as *const _ as *const _,
                size_of::<SOCKADDR_UN>() as _,
            );
            if err == SOCKET_ERROR {
                Err(wsa_error())
            } else {
                Ok(Self(s))
            }
        }
    }
}
