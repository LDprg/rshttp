use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ClientError {
    AddressNotFound(String),
    Network(std::io::Error),
}

impl Error for ClientError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::AddressNotFound(_) => None,
            Self::Network(error) => Some(error),
        }
    }
}

impl Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AddressNotFound(addr) => write!(f, "Address not found: {addr}"),
            Self::Network(error) => write!(f, "Address not found: {error}"),
        }
    }
}

impl From<std::io::Error> for ClientError {
    fn from(value: std::io::Error) -> Self {
        Self::Network(value)
    }
}
