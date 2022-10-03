use data_encoding::{DecodeError, Encoding, Specification};
use std::string::FromUtf8Error;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum EncodeError {
    #[error("Failed to decode base64")]
    EncodedIdDecodeError(#[from] DecodeError),
    #[error("Failed to decode ut8")]
    EncodedIdDecodeUtf8Error(#[from] FromUtf8Error),
}

/// Url safe base 64 encoder
pub struct Base64UrlEncoder;

impl Base64UrlEncoder {
    /// The used encoder. It is almost a BASE64URL, but padding is skipped
    fn encoder() -> Encoding {
        let mut spec = Specification::new();
        spec.symbols
            .push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_");
        spec.padding = None;
        spec.encoding().unwrap()
    }

    pub fn encode(&self, data: &[u8]) -> String {
        Self::encoder().encode(data)
    }

    pub fn encode_str(&self, data: &str) -> String {
        Self::encoder().encode(data.as_bytes())
    }

    pub fn decode(&self, data: &[u8]) -> Result<Vec<u8>, EncodeError> {
        Ok(Self::encoder().decode(data)?)
    }

    pub fn decode_str(&self, data: &[u8]) -> Result<String, EncodeError> {
        let data = Self::encoder().decode(data)?;
        Ok(String::from_utf8(data)?)
    }
}
