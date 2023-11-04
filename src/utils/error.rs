use std::error;
use std::fmt;

use crate::client;

#[derive(Debug)]
pub enum XIVAPIError {
    /// Occurs when the API has returned a non-success error code.
    Status(client::Response),
    /// Occurs if the HTTP response from XIVAPI was corrupt and
    /// reqwest could not parse it.
    Network(client::Error),
    /// Occurs if serde could not Deserialize the response.
    Parse(serde_json::Error),
}

impl From<client::Error> for XIVAPIError {
    fn from(e: client::Error) -> Self {
        XIVAPIError::Network(e)
    }
}

impl From<serde_json::Error> for XIVAPIError {
    fn from(e: serde_json::Error) -> Self {
        XIVAPIError::Parse(e)
    }
}

impl fmt::Display for XIVAPIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            XIVAPIError::Status(ref err) => write!(f, "Status error: {}", err.status()),
            XIVAPIError::Network(ref err) => err.fmt(f),
            XIVAPIError::Parse(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for XIVAPIError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            XIVAPIError::Status(_) => None,
            XIVAPIError::Network(ref err) => Some(err),
            XIVAPIError::Parse(ref err) => Some(err),
        }
    }
}
