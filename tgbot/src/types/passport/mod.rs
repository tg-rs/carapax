use crate::types::primitive::Integer;
use serde::Deserialize;

mod element;
mod error;

pub use self::{element::*, error::*};

/// Telegram Passport data shared with the bot by the user
#[derive(Clone, Debug, Deserialize)]
pub struct PassportData {
    /// Array with information about documents
    /// and other Telegram Passport elements
    /// that was shared with the bot
    pub data: Vec<EncryptedPassportElement>,
    /// Encrypted credentials required to decrypt the data
    pub credentials: EncryptedCredentials,
}

/// File uploaded to Telegram Passport
///
/// Currently all Telegram Passport files are in JPEG
/// format when decrypted and don't exceed 10MB
#[derive(Clone, Debug, Deserialize)]
pub struct PassportFile {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file
    ///
    /// It is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// File size
    pub file_size: Integer,
    /// Unix time when the file was uploaded
    pub file_date: Integer,
}

/// Data required for decrypting and authenticating EncryptedPassportElement
///
/// See the Telegram Passport Documentation for a complete description
/// of the data decryption and authentication processes
#[derive(Clone, Debug, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_data() {
        let data: PassportData = serde_json::from_value(serde_json::json!({
            "data": [
                {
                    "type": "address",
                    "data": "d",
                    "hash": "h"
                },
                {
                    "type": "bank_statement",
                    "files": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                    "translation": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                    "hash": "h"
                },
                {
                    "type": "driver_license",
                    "data": "d",
                    "front_side": {"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0},
                    "reverse_side": {"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0},
                    "selfie": {"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0},
                    "translation": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                    "hash": "h"
                },
                {
                    "type": "email",
                    "email": "u@h.z",
                    "hash": "h"
                },
                {
                    "type": "identity_card",
                    "data": "d",
                    "front_side": {"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0},
                    "reverse_side": {"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0},
                    "selfie": {"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0},
                    "translation": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                    "hash": "h"
                },
                {
                    "type": "internal_passport",
                    "data": "d",
                    "front_side": {"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0},
                    "selfie": {"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0},
                    "translation": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                    "hash": "h"
                },
                {
                    "type": "passport",
                    "data": "d",
                    "front_side": {"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0},
                    "selfie": {"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0},
                    "translation": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                    "hash": "h"
                },
                {
                    "type": "passport_registration",
                    "files": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                    "translation": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                    "hash": "h"
                },
                {
                    "type": "personal_details",
                    "data": "d",
                    "hash": "h"
                },
                {
                    "type": "phone_number",
                    "phone_number": "+79270000000",
                    "hash": "h"
                },
                {
                    "type": "rental_agreement",
                    "files": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                    "translation": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                    "hash": "h"
                },
                {
                    "type": "temporary_registration",
                    "files": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                    "translation": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                    "hash": "h"
                },
                {
                    "type": "utility_bill",
                    "files": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                    "translation": [{"file_id": "f", "file_unique_id": "uf", "file_size": 1, "file_date": 0}],
                    "hash": "h"
                }
            ],
            "credentials": {
                "data": "d",
                "hash": "h",
                "secret": "s"
            }
        }))
        .unwrap();
        assert_eq!(data.credentials.data, String::from("d"));
        assert_eq!(data.credentials.hash, String::from("h"));
        assert_eq!(data.credentials.secret, String::from("s"));
        assert_eq!(data.data.len(), 13);
    }
}
