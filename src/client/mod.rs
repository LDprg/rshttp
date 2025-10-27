use std::io::*;
use std::result::Result;

use crate::util::error::*;
use crate::util::socket::Socket;
use crate::util::url::Url;

pub fn get(url: &str) -> Result<String, ClientError> {
    let url = Url::from(url);

    println!("{:#?}", url);
    let mut socket = Socket::connect(&url)?;

    socket.write_all(format!("GET {} HTTP/1.1\r\n", url.path).as_bytes())?;
    socket.write_all(format!("Host: {}\r\n", url.host).as_bytes())?;
    socket.write_all("Connection: close\r\n".as_bytes())?;
    socket.write_all("\r\n".as_bytes())?;
    socket.write_all("\r\n".as_bytes())?;

    let mut buf = String::new();
    socket.read_to_string(&mut buf)?;

    Ok(buf)
}
