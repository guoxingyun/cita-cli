use hyper;
use serde_json;

/// Error summary information
#[derive(Debug, Fail)]
pub enum ToolError {
    /// IO error
    #[fail(display = "std's io error: {}", _0)]
    Stdio(::std::io::Error),
    /// Parsing json data error
    #[fail(display = "serde_json error: {}", _0)]
    SerdeJson(serde_json::error::Error),
    /// Hyper error
    #[fail(display = "Hyper error: {}", _0)]
    Hyper(hyper::Error),
    /// ABI error
    #[fail(display = "ABI error: {}", _0)]
    Abi(String),
}
