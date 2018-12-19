use crate::types::primitive::Integer;

mod element;
mod error;
#[cfg(test)]
mod tests;

pub use self::element::*;
pub use self::error::*;

/// Contains information about
/// Telegram Passport data
/// shared with the bot by the user
#[derive(Debug, Deserialize)]
pub struct PassportData {
    /// Array with information about documents
    /// and other Telegram Passport elements
    /// that was shared with the bot
    pub data: Vec<EncryptedPassportElement>,
    /// Encrypted credentials required to decrypt the data
    pub credentials: EncryptedCredentials,
}

/// This object represents a file uploaded to Telegram Passport
/// Currently all Telegram Passport files are in JPEG
/// format when decrypted and don't exceed 10MB
#[derive(Debug, Deserialize)]
pub struct PassportFile {
    /// Unique identifier for this file
    pub file_id: String,
    /// File size
    pub file_size: Integer,
    /// Unix time when the file was uploaded
    pub file_date: Integer,
}

/// Contains data required for decrypting
/// and authenticating EncryptedPassportElement
/// See the Telegram Passport Documentation for a complete description
/// of the data decryption and authentication processes
#[derive(Debug, Deserialize)]
pub struct EncryptedCredentials {
    /// Base64-encoded encrypted JSON-serialized data
    /// with unique user's payload,
    /// data hashes and secrets required
    /// for EncryptedPassportElement decryption and authentication
    pub data: String,
    /// Base64-encoded data hash for data authentication
    pub hash: String,
    /// Base64-encoded secret, encrypted
    /// with the bot's public RSA key,
    /// required for data decryption
    pub secret: String,
}
