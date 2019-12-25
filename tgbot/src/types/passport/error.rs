use crate::types::passport::element::EncryptedPassportElementKind;
use serde::Serialize;
use std::{error::Error as StdError, fmt};

/// Error in the Telegram Passport element which was submitted that should be resolved by the user
#[derive(Clone, Debug, Serialize)]
pub struct PassportElementError {
    #[serde(flatten)]
    kind: PassportElementErrorKind,
}

impl PassportElementError {
    /// Represents an issue in one of the data fields that was provided by the user
    /// The error is considered resolved when the field's value changes
    ///
    /// # Arguments
    ///
    /// * kind - The section of the user's Telegram Passport which has the error,
    ///          one of “personal_details”, “passport”, “driver_license”,
    ///          “identity_card”, “internal_passport”, “address”
    /// * field_name - Name of the data field which has the error
    /// * data_hash - Base64-encoded data hash
    /// * message - Error message
    pub fn data_field<S: Into<String>>(
        kind: EncryptedPassportElementKind,
        field_name: S,
        data_hash: S,
        message: S,
    ) -> Result<Self, UnexpectedEncryptedPassportElementKind> {
        use self::EncryptedPassportElementKind::*;
        match kind {
            Address | DriverLicense | IdentityCard | InternalPassport | Passport | PersonalDetails => {
                Ok(PassportElementError {
                    kind: PassportElementErrorKind::DataField {
                        kind,
                        field_name: field_name.into(),
                        data_hash: data_hash.into(),
                        message: message.into(),
                    },
                })
            }
            _ => Err(UnexpectedEncryptedPassportElementKind(kind)),
        }
    }

    /// Represents an issue with the front side of a document
    /// The error is considered resolved when the file
    /// with the front side of the document changes
    ///
    /// # Arguments
    ///
    /// * kind - The section of the user's Telegram Passport which has the issue,
    ///          one of “passport”, “driver_license”, “identity_card”, “internal_passport”
    /// * file_hash - Base64-encoded hash of the file with the front side of the document
    /// * message -  Error message
    pub fn front_side<S: Into<String>>(
        kind: EncryptedPassportElementKind,
        file_hash: S,
        message: S,
    ) -> Result<Self, UnexpectedEncryptedPassportElementKind> {
        use self::EncryptedPassportElementKind::*;
        match kind {
            DriverLicense | IdentityCard | InternalPassport | Passport => Ok(PassportElementError {
                kind: PassportElementErrorKind::FrontSide {
                    kind,
                    file_hash: file_hash.into(),
                    message: message.into(),
                },
            }),
            _ => Err(UnexpectedEncryptedPassportElementKind(kind)),
        }
    }

    /// Represents an issue with the reverse side of a document
    /// The error is considered resolved when the
    /// file with reverse side of the document changes
    ///
    /// # Arguments
    ///
    /// * kind - The section of the user's Telegram Passport which has the issue,
    ///          one of “driver_license”, “identity_card”
    /// * file_hash - Base64-encoded hash of the file with the reverse side of the document
    /// * message - Error message
    pub fn reverse_side<S: Into<String>>(
        kind: EncryptedPassportElementKind,
        file_hash: S,
        message: S,
    ) -> Result<Self, UnexpectedEncryptedPassportElementKind> {
        use self::EncryptedPassportElementKind::*;
        match kind {
            DriverLicense | IdentityCard => Ok(PassportElementError {
                kind: PassportElementErrorKind::ReverseSide {
                    kind,
                    file_hash: file_hash.into(),
                    message: message.into(),
                },
            }),
            _ => Err(UnexpectedEncryptedPassportElementKind(kind)),
        }
    }

    /// Represents an issue with the selfie with a document
    /// The error is considered resolved when the file with the selfie changes
    ///
    /// # Arguments
    ///
    /// * kind - The section of the user's Telegram Passport which has the issue,
    ///          one of “passport”, “driver_license”, “identity_card”, “internal_passport”
    /// * file_hash - Base64-encoded hash of the file with the selfie
    /// * message - Error message
    pub fn selfie<S: Into<String>>(
        kind: EncryptedPassportElementKind,
        file_hash: S,
        message: S,
    ) -> Result<Self, UnexpectedEncryptedPassportElementKind> {
        use self::EncryptedPassportElementKind::*;
        match kind {
            DriverLicense | IdentityCard | InternalPassport | Passport => Ok(PassportElementError {
                kind: PassportElementErrorKind::Selfie {
                    kind,
                    file_hash: file_hash.into(),
                    message: message.into(),
                },
            }),
            _ => Err(UnexpectedEncryptedPassportElementKind(kind)),
        }
    }
    /// Represents an issue with a document scan
    /// The error is considered resolved when
    /// the file with the document scan changes
    ///
    /// # Arguments
    ///
    /// * kind - The section of the user's Telegram Passport which has the issue,
    ///          one of “utility_bill”, “bank_statement”, “rental_agreement”,
    ///          “passport_registration”, “temporary_registration”
    /// * file_hash - Base64-encoded hash of the file with the selfie
    /// * message - Error message
    pub fn file<S: Into<String>>(
        kind: EncryptedPassportElementKind,
        file_hash: S,
        message: S,
    ) -> Result<Self, UnexpectedEncryptedPassportElementKind> {
        use self::EncryptedPassportElementKind::*;
        match kind {
            BankStatement | PassportRegistration | RentalAgreement | TemporaryRegistration | UtilityBill => {
                Ok(PassportElementError {
                    kind: PassportElementErrorKind::File {
                        kind,
                        file_hash: file_hash.into(),
                        message: message.into(),
                    },
                })
            }
            _ => Err(UnexpectedEncryptedPassportElementKind(kind)),
        }
    }

    /// Represents an issue with a list of scans
    /// The error is considered resolved when
    /// the list of files containing the scans changes
    ///
    /// # Arguments
    ///
    /// * kind - The section of the user's Telegram Passport which has the issue, one of
    ///          “utility_bill”, “bank_statement”, “rental_agreement”,
    ///          “passport_registration”, “temporary_registration”
    /// * file_hashes - List of base64-encoded file hashes
    /// * message - Error message
    pub fn files<S: Into<String>>(
        kind: EncryptedPassportElementKind,
        file_hashes: Vec<String>,
        message: S,
    ) -> Result<Self, UnexpectedEncryptedPassportElementKind> {
        use self::EncryptedPassportElementKind::*;
        match kind {
            BankStatement | PassportRegistration | RentalAgreement | TemporaryRegistration | UtilityBill => {
                Ok(PassportElementError {
                    kind: PassportElementErrorKind::Files {
                        kind,
                        file_hashes,
                        message: message.into(),
                    },
                })
            }
            _ => Err(UnexpectedEncryptedPassportElementKind(kind)),
        }
    }

    /// Represents an issue with one of the files that constitute
    /// the translation of a document
    /// The error is considered resolved when the file changes
    ///
    /// # Arguments
    ///
    /// * kind - Type of element of the user's Telegram Passport which has the issue,
    ///          one of “passport”, “driver_license”, “identity_card”,
    ///          “internal_passport”, “utility_bill”, “bank_statement”,
    ///          “rental_agreement”, “passport_registration”, “temporary_registration”
    /// * file_hash - Base64-encoded hash of the file with the selfie
    /// * message - Error message
    pub fn translation_file<S: Into<String>>(
        kind: EncryptedPassportElementKind,
        file_hash: S,
        message: S,
    ) -> Result<Self, UnexpectedEncryptedPassportElementKind> {
        use self::EncryptedPassportElementKind::*;
        match kind {
            BankStatement
            | DriverLicense
            | IdentityCard
            | InternalPassport
            | Passport
            | PassportRegistration
            | RentalAgreement
            | TemporaryRegistration
            | UtilityBill => Ok(PassportElementError {
                kind: PassportElementErrorKind::TranslationFile {
                    kind,
                    file_hash: file_hash.into(),
                    message: message.into(),
                },
            }),
            _ => Err(UnexpectedEncryptedPassportElementKind(kind)),
        }
    }

    /// Represents an issue with the translated version of a document
    /// The error is considered resolved when a file
    /// with the document translation change
    ///
    /// # Arguments
    ///
    /// * kind - Type of element of the user's Telegram Passport which has the issue, one of
    ///          “passport”, “driver_license”, “identity_card”,
    ///          “internal_passport”, “utility_bill”, “bank_statement”,
    ///          “rental_agreement”, “passport_registration”, “temporary_registration”
    /// * file_hashes - List of base64-encoded file hashes
    /// * message - Error message
    pub fn translation_files<S: Into<String>>(
        kind: EncryptedPassportElementKind,
        file_hashes: Vec<String>,
        message: S,
    ) -> Result<Self, UnexpectedEncryptedPassportElementKind> {
        use self::EncryptedPassportElementKind::*;
        match kind {
            BankStatement
            | DriverLicense
            | IdentityCard
            | InternalPassport
            | Passport
            | PassportRegistration
            | RentalAgreement
            | TemporaryRegistration
            | UtilityBill => Ok(PassportElementError {
                kind: PassportElementErrorKind::TranslationFiles {
                    kind,
                    file_hashes,
                    message: message.into(),
                },
            }),
            _ => Err(UnexpectedEncryptedPassportElementKind(kind)),
        }
    }

    /// Represents an issue in an unspecified place
    /// The error is considered resolved when new data is added
    ///
    /// # Arguments
    ///
    /// * kind - Type of element of the user's Telegram Passport which has the issue
    /// * element_hash - Base64-encoded element hash
    /// * message - Error message
    pub fn unspecified<S: Into<String>>(kind: EncryptedPassportElementKind, element_hash: S, message: S) -> Self {
        PassportElementError {
            kind: PassportElementErrorKind::Unspecified {
                kind,
                element_hash: element_hash.into(),
                message: message.into(),
            },
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "source")]
enum PassportElementErrorKind {
    #[serde(rename = "data")]
    DataField {
        #[serde(rename = "type")]
        kind: EncryptedPassportElementKind,
        field_name: String,
        data_hash: String,
        message: String,
    },
    #[serde(rename = "front_side")]
    FrontSide {
        #[serde(rename = "type")]
        kind: EncryptedPassportElementKind,
        file_hash: String,
        message: String,
    },
    #[serde(rename = "reverse_side")]
    ReverseSide {
        #[serde(rename = "type")]
        kind: EncryptedPassportElementKind,
        file_hash: String,
        message: String,
    },
    #[serde(rename = "selfie")]
    Selfie {
        #[serde(rename = "type")]
        kind: EncryptedPassportElementKind,
        file_hash: String,
        message: String,
    },
    #[serde(rename = "file")]
    File {
        #[serde(rename = "type")]
        kind: EncryptedPassportElementKind,
        file_hash: String,
        message: String,
    },
    #[serde(rename = "files")]
    Files {
        #[serde(rename = "type")]
        kind: EncryptedPassportElementKind,
        file_hashes: Vec<String>,
        message: String,
    },
    #[serde(rename = "translation_file")]
    TranslationFile {
        #[serde(rename = "type")]
        kind: EncryptedPassportElementKind,
        file_hash: String,
        message: String,
    },
    #[serde(rename = "translation_files")]
    TranslationFiles {
        #[serde(rename = "type")]
        kind: EncryptedPassportElementKind,
        file_hashes: Vec<String>,
        message: String,
    },
    #[serde(rename = "unspecified")]
    Unspecified {
        #[serde(rename = "type")]
        kind: EncryptedPassportElementKind,
        element_hash: String,
        message: String,
    },
}

/// Unexpected encrypted passport element kind
#[derive(Clone, Debug)]
pub struct UnexpectedEncryptedPassportElementKind(EncryptedPassportElementKind);

impl StdError for UnexpectedEncryptedPassportElementKind {}

impl fmt::Display for UnexpectedEncryptedPassportElementKind {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "unexpected element kind: {:?}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_error() {
        let err = PassportElementError::data_field(
            EncryptedPassportElementKind::Address,
            "address",
            "data_hash",
            "bad address",
        )
        .unwrap();
        assert_eq!(
            serde_json::to_value(&err).unwrap(),
            serde_json::json!({"source":"data","type":"address","field_name":"address","data_hash":"data_hash","message":"bad address"})
        );

        let err =
            PassportElementError::front_side(EncryptedPassportElementKind::DriverLicense, "file_hash", "bad file")
                .unwrap();
        assert_eq!(
            serde_json::to_value(&err).unwrap(),
            serde_json::json!({"source":"front_side","type":"driver_license","file_hash":"file_hash","message":"bad file"})
        );

        let err =
            PassportElementError::reverse_side(EncryptedPassportElementKind::DriverLicense, "file_hash", "bad file")
                .unwrap();
        assert_eq!(
            serde_json::to_value(&err).unwrap(),
            serde_json::json!({"source":"reverse_side","type":"driver_license","file_hash":"file_hash","message":"bad file"})
        );

        let err =
            PassportElementError::selfie(EncryptedPassportElementKind::DriverLicense, "file_hash", "bad file").unwrap();
        assert_eq!(
            serde_json::to_value(&err).unwrap(),
            serde_json::json!({"source":"selfie","type":"driver_license","file_hash":"file_hash","message":"bad file"})
        );

        let err =
            PassportElementError::file(EncryptedPassportElementKind::BankStatement, "file_hash", "bad file").unwrap();
        assert_eq!(
            serde_json::to_value(&err).unwrap(),
            serde_json::json!({"source":"file","type":"bank_statement","file_hash":"file_hash","message":"bad file"})
        );

        let err = PassportElementError::files(
            EncryptedPassportElementKind::BankStatement,
            vec![String::from("file_hash")],
            "bad file",
        )
        .unwrap();
        assert_eq!(
            serde_json::to_value(&err).unwrap(),
            serde_json::json!({"source":"files","type":"bank_statement","file_hashes":["file_hash"],"message":"bad file"})
        );

        let err = PassportElementError::translation_file(
            EncryptedPassportElementKind::BankStatement,
            "file_hash",
            "bad file",
        )
        .unwrap();
        assert_eq!(
            serde_json::to_value(&err).unwrap(),
            serde_json::json!({"source":"translation_file","type":"bank_statement","file_hash":"file_hash","message":"bad file"})
        );

        let err = PassportElementError::translation_files(
            EncryptedPassportElementKind::BankStatement,
            vec![String::from("file_hash")],
            "bad file",
        )
        .unwrap();
        assert_eq!(
            serde_json::to_value(&err).unwrap(),
            serde_json::json!({"source":"translation_files","type":"bank_statement","file_hashes":["file_hash"],"message":"bad file"})
        );

        let err =
            PassportElementError::unspecified(EncryptedPassportElementKind::BankStatement, "element_hash", "bad file");
        assert_eq!(
            serde_json::to_value(&err).unwrap(),
            serde_json::json!({"source":"unspecified","type":"bank_statement","element_hash":"element_hash","message":"bad file"})
        );
    }

    #[test]
    fn create_error_accepts_kind() {
        use self::EncryptedPassportElementKind::*;
        for (kind, flag) in vec![
            (Address, true),
            (BankStatement, false),
            (DriverLicense, true),
            (Email, false),
            (IdentityCard, true),
            (InternalPassport, true),
            (Passport, true),
            (PassportRegistration, false),
            (PersonalDetails, true),
            (PhoneNumber, false),
            (RentalAgreement, false),
            (TemporaryRegistration, false),
            (UtilityBill, false),
        ] {
            let err = PassportElementError::data_field(kind, "address", "data_hash", "bad address");
            assert!(if flag { err.is_ok() } else { err.is_err() });
        }

        for (kind, flag) in vec![
            (Address, false),
            (BankStatement, false),
            (DriverLicense, true),
            (Email, false),
            (IdentityCard, true),
            (InternalPassport, true),
            (Passport, true),
            (PassportRegistration, false),
            (PersonalDetails, false),
            (PhoneNumber, false),
            (RentalAgreement, false),
            (TemporaryRegistration, false),
            (UtilityBill, false),
        ] {
            let err = PassportElementError::front_side(kind, "file_hash", "bad file");
            assert!(if flag { err.is_ok() } else { err.is_err() });
        }

        for (kind, flag) in vec![
            (Address, false),
            (BankStatement, false),
            (DriverLicense, true),
            (Email, false),
            (IdentityCard, true),
            (InternalPassport, false),
            (Passport, false),
            (PassportRegistration, false),
            (PersonalDetails, false),
            (PhoneNumber, false),
            (RentalAgreement, false),
            (TemporaryRegistration, false),
            (UtilityBill, false),
        ] {
            let err = PassportElementError::reverse_side(kind, "file_hash", "bad file");
            assert!(if flag { err.is_ok() } else { err.is_err() });
        }

        for (kind, flag) in vec![
            (Address, false),
            (BankStatement, false),
            (DriverLicense, true),
            (Email, false),
            (IdentityCard, true),
            (InternalPassport, true),
            (Passport, true),
            (PassportRegistration, false),
            (PersonalDetails, false),
            (PhoneNumber, false),
            (RentalAgreement, false),
            (TemporaryRegistration, false),
            (UtilityBill, false),
        ] {
            let err = PassportElementError::selfie(kind, "file_hash", "bad file");
            assert!(if flag { err.is_ok() } else { err.is_err() });
        }

        for (kind, flag) in vec![
            (Address, false),
            (BankStatement, true),
            (DriverLicense, false),
            (Email, false),
            (IdentityCard, false),
            (InternalPassport, false),
            (Passport, false),
            (PassportRegistration, true),
            (PersonalDetails, false),
            (PhoneNumber, false),
            (RentalAgreement, true),
            (TemporaryRegistration, true),
            (UtilityBill, true),
        ] {
            let err = PassportElementError::file(kind, "file_hash", "bad file");
            assert!(if flag { err.is_ok() } else { err.is_err() });
        }

        for (kind, flag) in vec![
            (Address, false),
            (BankStatement, true),
            (DriverLicense, false),
            (Email, false),
            (IdentityCard, false),
            (InternalPassport, false),
            (Passport, false),
            (PassportRegistration, true),
            (PersonalDetails, false),
            (PhoneNumber, false),
            (RentalAgreement, true),
            (TemporaryRegistration, true),
            (UtilityBill, true),
        ] {
            let err = PassportElementError::files(kind, vec![String::from("file_hash")], "bad file");
            assert!(if flag { err.is_ok() } else { err.is_err() });
        }

        for (kind, flag) in vec![
            (Address, false),
            (BankStatement, true),
            (DriverLicense, true),
            (Email, false),
            (IdentityCard, true),
            (InternalPassport, true),
            (Passport, true),
            (PassportRegistration, true),
            (PersonalDetails, false),
            (PhoneNumber, false),
            (RentalAgreement, true),
            (TemporaryRegistration, true),
            (UtilityBill, true),
        ] {
            let err = PassportElementError::translation_file(kind, "file_hash", "bad file");
            assert!(if flag { err.is_ok() } else { err.is_err() });
        }

        for (kind, flag) in vec![
            (Address, false),
            (BankStatement, true),
            (DriverLicense, true),
            (Email, false),
            (IdentityCard, true),
            (InternalPassport, true),
            (Passport, true),
            (PassportRegistration, true),
            (PersonalDetails, false),
            (PhoneNumber, false),
            (RentalAgreement, true),
            (TemporaryRegistration, true),
            (UtilityBill, true),
        ] {
            let err = PassportElementError::translation_files(kind, vec![String::from("file_hash")], "bad file");
            assert!(if flag { err.is_ok() } else { err.is_err() });
        }
    }
}
