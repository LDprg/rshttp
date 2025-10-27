use std::io::*;
use std::net::*;
use std::sync::Arc;

use rustls::RootCertStore;
use rustls::pki_types::ServerName;

use crate::util::url::Scheme;
use crate::util::url::Url;

pub struct Socket {
    socket: Option<TcpStream>,
    tls: Option<rustls::StreamOwned<rustls::ClientConnection, TcpStream>>,
}

impl Socket {
    pub fn connect(url: &Url) -> Result<Self> {
        match url.scheme {
            Scheme::HTTP => Self::connect_plain(url),
            Scheme::HTTPS => Self::connect_tls(url),
        }
    }

    fn connect_plain(url: &Url) -> Result<Self> {
        let socket = TcpStream::connect(format!("{}:{}", url.host, url.port))?;

        Ok(Self {
            socket: Some(socket),
            tls: None,
        })
    }

    fn connect_tls(url: &Url) -> Result<Self> {
        let root_store = RootCertStore {
            roots: webpki_roots::TLS_SERVER_ROOTS.into(),
        };

        let mut config = rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        // Allow using SSLKEYLOGFILE.
        config.key_log = Arc::new(rustls::KeyLogFile::new());

        config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

        let server_name: ServerName = url.host.to_string().try_into().unwrap();

        let conn = rustls::ClientConnection::new(Arc::new(config), server_name).unwrap();
        let socket = TcpStream::connect(format!("{}:{}", url.host, url.port))?;
        let tls = rustls::StreamOwned::new(conn, socket);

        Ok(Self {
            socket: None,
            tls: Some(tls),
        })
    }
}

impl Write for Socket {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if let Some(tls) = &mut self.tls {
            tls.write(buf)
        } else if let Some(socket) = &mut self.socket {
            socket.write(buf)
        } else {
            panic!("Sockets broken");
        }
    }

    fn flush(&mut self) -> Result<()> {
        if let Some(tls) = &mut self.tls {
            tls.flush()
        } else if let Some(socket) = &mut self.socket {
            socket.flush()
        } else {
            panic!("Sockets broken");
        }
    }
}

impl Read for Socket {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if let Some(tls) = &mut self.tls {
            tls.read(buf)
        } else if let Some(socket) = &mut self.socket {
            socket.read(buf)
        } else {
            panic!("Sockets broken");
        }
    }
}
