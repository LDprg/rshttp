use std::io::*;
use std::net::*;
use std::result::Result;

use crate::util::error::*;
use crate::util::url::Url;

pub fn get(url: &str) -> Result<String, ClientError> {
    let url = Url::from(url);
    let mut ip = format!("{}:{}", url.host, url.port).to_socket_addrs()?;

    if let Some(ip) = ip.next() {
        println!("{}", ip);
        let mut socket = TcpStream::connect(ip)?;

        socket.write(format!("GET {} HTTP/1.1\r\n", url.path).as_bytes())?;
        socket.write(format!("Host: {}\r\n", url.host).as_bytes())?;
        socket.write("Connection: close\r\n".as_bytes())?;
        socket.write("\r\n".as_bytes())?;
        socket.write("\r\n".as_bytes())?;

        let mut buf = String::new();
        socket.read_to_string(&mut buf)?;

        return Ok(buf);
    }

    Err(ClientError::AddressNotFound(url.host.to_string()))
}
