use std::io;

use windows::Win32::Networking::WinSock::{self, WSA_ERROR, WSADATA};

pub fn startup() -> io::Result<()> {
    use WinSock::{WSAEFAULT, WSAEINPROGRESS, WSAEPROCLIM, WSASYSNOTREADY, WSAVERNOTSUPPORTED};
    let mut wsa_data = WSADATA::default();
    match WSA_ERROR(unsafe { WinSock::WSAStartup(0x202, &mut wsa_data) }) {
        WSA_ERROR(0) => Ok(()),
        WSASYSNOTREADY => Err(io::Error::other("Network subsystem not ready")),
        WSAVERNOTSUPPORTED => Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Winsock version not supported",
        )),
        WSAEINPROGRESS => Err(io::Error::new(
            io::ErrorKind::WouldBlock,
            "Blocking operation in progress",
        )),
        WSAEPROCLIM => Err(io::Error::other("Too many tasks")),
        WSAEFAULT => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid parameter",
        )),
        _ => Err(io::Error::other("Unknown WSAStartup error")),
    }
}
