use std::io;

use futures::*;
use win_uds::net::{AsyncListener, AsyncStream, UnixListener, UnixStream};

#[tokio::test]
async fn async_test() -> io::Result<()> {
    let _ = std::fs::remove_file("test.sock");
    let listener = AsyncListener::bind("test.sock").unwrap();
    let srv = tokio::spawn(async move {
        let (mut s, _) = listener.accept().unwrap();
        let mut buf = [0u8; 1024];
        s.read(&mut buf).await.unwrap();
        println!("{}", String::from_utf8_lossy(&buf));
        s.write(&mut buf).await.unwrap();
        s.close().await.unwrap();
    });
    let mut cli = AsyncStream::connect("test.sock").unwrap();
    cli.write_all(b"Hello").await.unwrap();
    let mut buf = Vec::new();
    cli.read_to_end(&mut buf).await.unwrap();
    println!("{}", String::from_utf8_lossy(&buf));
    srv.await.unwrap();
    cli.close().await.unwrap();
    std::fs::remove_file("test.sock")
}
use io::Read;
#[test]
fn no_bloking_test() {
    let _ = std::fs::remove_file("test.sock");
    let l = UnixListener::bind("test.sock").unwrap();
    l.set_nonblocking(true).unwrap();
    let mut s = UnixStream::connect("test.sock").unwrap();
    s.set_nonblocking(true).unwrap();
    let mut buf = [0u8; 1024];
    assert_eq!(
        s.read(&mut buf).unwrap_err().kind(),
        io::ErrorKind::WouldBlock
    );
    let _ = std::fs::remove_file("test.sock");
}
