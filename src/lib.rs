#[test]
fn test_google() {
    use mio::net::TcpStream;
    use native_tls::{HandshakeError, TlsConnector};
    use std::net::ToSocketAddrs;

    let stream =
        TcpStream::connect("google.com:443".to_socket_addrs().unwrap().next().unwrap()).unwrap();
    let connector = TlsConnector::new().unwrap();
    let mut stream = connector.connect("google.com", stream);

    while let Err(HandshakeError::WouldBlock(mid_handshake)) = stream {
        stream = mid_handshake.handshake();
    }

    stream.expect("Fails with NotConnected on windows");
}
