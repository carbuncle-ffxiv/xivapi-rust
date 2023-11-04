use thiserror::Error;

use crate::client;

#[derive(Debug, Error)]
pub enum XIVAPIError {
    /// Occurs if the HTTP response from XIVAPI was corrupt and
    /// reqwest could not parse it.
    #[error("An error occurred when processing a request: {0}")]
    ClientError(#[from] client::Error),
    /// Occurs if serde could not Deserialize the response.
    #[error("An error occurred when parsing a response: {0}")]
    Parse(#[from] serde_json::Error),
}
