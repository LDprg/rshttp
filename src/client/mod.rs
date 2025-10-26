use std::io::*;
use std::net::*;
use std::result::Result;

pub fn get(addr: &str) -> Result<String, Error> {
    let mut ip = addr.to_socket_addrs()?;
    if let Some(ip) = ip.next() {
        println!("{}", ip);
        let mut socket = TcpStream::connect(ip)?;

        socket.write("GET / HTTP/1.1\r\n".as_bytes())?;
        socket.write("Host: example.com\r\n".as_bytes())?;
        socket.write("Connection: close\r\n".as_bytes())?;
        socket.write("\r\n".as_bytes())?;
        socket.write("\r\n".as_bytes())?;

        let mut buf = String::new();
        socket.read_to_string(&mut buf)?;

        return Ok(buf);
    }

    Ok("Error".to_string())
}
