use crate::types::primitive::Integer;

/// Contains information about
/// Telegram Passport data
/// shared with the bot by the user.
#[derive(Debug)]
pub struct PassportData {
    /// Array with information about documents
    /// and other Telegram Passport elements
    /// that was shared with the bot
    pub data: Vec<EncryptedPassportElement>,
    /// Encrypted credentials required to decrypt the data
    pub credentials: EncryptedCredentials,
}

/// This object represents a file uploaded to Telegram Passport.
/// Currently all Telegram Passport files are in JPEG
/// format when decrypted and don't exceed 10MB.
#[derive(Debug)]
pub struct PassportFile {
    /// Unique identifier for this file
    pub file_id: String,
    /// File size
    pub file_size: Integer,
    /// Unix time when the file was uploaded
    pub file_date: Integer,
}

/// Contains information about documents or
/// other Telegram Passport elements
/// shared with the bot by the user.
#[derive(Debug)]
pub struct EncryptedPassportElement {
    /// Element type.
    /// One of
    /// “personal_details”, “passport”, “driver_license”,
    /// “identity_card”, “internal_passport”, “address”,
    /// “utility_bill”, “bank_statement”, “rental_agreement”,
    /// “passport_registration”, “temporary_registration”,
    /// “phone_number”, “email”.
    pub kind: String, // TODO: rename to type
    /// Base64-encoded encrypted
    /// Telegram Passport element data provided by the user,
    /// available for
    /// “personal_details”, “passport”, “driver_license”,
    /// “identity_card”, “internal_passport” and “address” types.
    /// Can be decrypted and verified using
    /// the accompanying EncryptedCredentials.
    pub data: Option<String>,
    /// User's verified phone number, available only for “phone_number” type
    pub phone_number: Option<String>,
    /// User's verified email address, available only for “email” type
    pub email: Option<String>,
    /// Array of encrypted files with
    /// documents provided by the user,
    /// available for “utility_bill”,
    /// “bank_statement”, “rental_agreement”,
    /// “passport_registration” and “temporary_registration” types.
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials.
    pub files: Option<Vec<PassportFile>>,
    /// Encrypted file with the front side
    /// of the document, provided by the user.
    /// Available for “passport”, “driver_license”,
    /// “identity_card” and “internal_passport”.
    /// The file can be decrypted and verified
    /// using the accompanying EncryptedCredentials.
    pub front_side: Option<PassportFile>,
    /// Encrypted file with the reverse side of the document,
    /// provided by the user.
    /// Available for “driver_license” and “identity_card”.
    /// The file can be decrypted and verified
    /// using the accompanying EncryptedCredentials.
    pub reverse_side: Option<PassportFile>,
    /// Encrypted file with the selfie of the user
    /// holding a document, provided by the user;
    /// available for “passport”, “driver_license”, “identity_card” and “internal_passport”.
    /// The file can be decrypted and verified
    /// using the accompanying EncryptedCredentials.
    pub selfie: Option<PassportFile>,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user.
    /// Available if requested for “passport”,
    /// “driver_license”, “identity_card”,
    /// “internal_passport”, “utility_bill”,
    /// “bank_statement”, “rental_agreement”,
    /// “passport_registration” and “temporary_registration” types.
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials.
    pub translation: Option<Vec<PassportFile>>,
    /// Base64-encoded element hash for
    /// using in PassportElementErrorUnspecified
    pub hash: String,
}

/// Contains data required for decrypting
/// and authenticating EncryptedPassportElement.
/// See the Telegram Passport Documentation for a complete description
/// of the data decryption and authentication processes.
#[derive(Debug)]
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

/// This object represents an error in the Telegram Passport
/// element which was submitted that should
/// be resolved by the user.
#[derive(Debug)]
pub enum PassportElementError {
    /// Represents an issue in one of the data fields that was provided by the user.
    /// The error is considered resolved when the field's value changes.
    DataField(PassportElementErrorDataField),
    /// Represents an issue with the front side of a document.
    /// The error is considered resolved when the file
    /// with the front side of the document changes.
    FrontSide(PassportElementErrorFrontSide),
    /// Represents an issue with the reverse side of a document.
    /// The error is considered resolved when the
    /// file with reverse side of the document changes.
    ReverseSide(PassportElementErrorReverseSide),
    /// Represents an issue with the selfie with a document.
    /// The error is considered resolved when the file with the selfie changes.
    Selfie(PassportElementErrorSelfie),
    /// Represents an issue with a document scan.
    /// The error is considered resolved when
    /// the file with the document scan changes.
    File(PassportElementErrorFile),
    /// Represents an issue with a list of scans.
    /// The error is considered resolved when
    /// the list of files containing the scans changes.
    Files(PassportElementErrorFiles),
    /// Represents an issue with one of the files that constitute
    /// the translation of a document.
    /// The error is considered resolved when the file changes.
    TranslationFile(PassportElementErrorTranslationFile),
    /// Represents an issue with the translated version of a document.
    /// The error is considered resolved when a file
    /// with the document translation change.
    TranslationFiles(PassportElementErrorTranslationFiles),
    /// Represents an issue in an unspecified place.
    /// The error is considered resolved when new data is added.
    Unspecified(PassportElementErrorUnspecified),
}

/// Represents an issue in one of the data fields that was provided by the user.
/// The error is considered resolved when the field's value changes.
#[derive(Debug)]
pub struct PassportElementErrorDataField {
    /// Error source, must be data
    pub source: String,
    /// The section of the user's Telegram Passport which has the error,
    /// one of “personal_details”, “passport”, “driver_license”,
    /// “identity_card”, “internal_passport”, “address”
    pub kind: String, // TODO: rename to type
    /// Name of the data field which has the error
    pub field_name: String,
    /// Base64-encoded data hash
    pub data_hash: String,
    /// Error message
    pub message: String,
}

/// Represents an issue with the front side of a document.
/// The error is considered resolved when the file
/// with the front side of the document changes.
#[derive(Debug)]
pub struct PassportElementErrorFrontSide {
    /// Error source, must be front_side
    pub source: String,
    /// The section of the user's Telegram Passport
    /// which has the issue, one of
    /// “passport”, “driver_license”,
    /// “identity_card”, “internal_passport”
    pub kind: String, // TODO: rename to type
    /// Base64-encoded hash of the file with the front side of the document
    pub file_hash: String,
    /// Error message
    pub message: String,
}

/// Represents an issue with the reverse side of a document.
/// The error is considered resolved when the
/// file with reverse side of the document changes.
#[derive(Debug)]
pub struct PassportElementErrorReverseSide {
    /// Error source, must be reverse_side
    pub source: String,
    /// The section of the user's Telegram Passport
    /// which has the issue, one of “driver_license”, “identity_card”
    pub kind: String, // TODO: rename to type
    /// Base64-encoded hash of the file with the reverse side of the document
    pub file_hash: String,
    /// Error message
    pub message: String,
}

/// Represents an issue with the selfie with a document.
/// The error is considered resolved when the file with the selfie changes.
#[derive(Debug)]
pub struct PassportElementErrorSelfie {
    /// Error source, must be selfie
    pub source: String,
    /// The section of the user's Telegram Passport
    /// which has the issue, one of
    /// “passport”, “driver_license”,
    /// “identity_card”, “internal_passport”
    pub kind: String, // TODO: rename to type
    /// Base64-encoded hash of the file with the selfie
    pub file_hash: String,
    /// Error message
    pub message: String,
}

/// Represents an issue with a document scan.
/// The error is considered resolved when
/// the file with the document scan changes.
#[derive(Debug)]
pub struct PassportElementErrorFile {
    /// Error source, must be file
    pub source: String,
    /// The section of the user's Telegram Passport
    /// which has the issue, one of
    /// “utility_bill”, “bank_statement”,
    /// “rental_agreement”, “passport_registration”,
    /// “temporary_registration”
    pub kind: String, // TODO: rename to type
    /// Base64-encoded file hash
    pub file_hash: String,
    /// Error message
    pub message: String,
}

/// Represents an issue with a list of scans.
/// The error is considered resolved when
/// the list of files containing the scans changes.
#[derive(Debug)]
pub struct PassportElementErrorFiles {
    /// Error source, must be files
    pub source: String,
    /// The section of the user's Telegram Passport
    /// which has the issue, one of
    /// “utility_bill”, “bank_statement”,
    /// “rental_agreement”, “passport_registration”,
    /// “temporary_registration”
    pub kind: String, // TODO: rename to type
    /// List of base64-encoded file hashes
    pub file_hashes: Vec<String>,
    /// Error message
    pub message: String,
}

/// Represents an issue with one of the files that constitute
/// the translation of a document.
/// The error is considered resolved when the file changes.
#[derive(Debug)]
pub struct PassportElementErrorTranslationFile {
    /// Error source, must be translation_file
    pub source: String,
    /// Type of element of the user's
    /// Telegram Passport which has the issue,
    /// one of “passport”, “driver_license”, “identity_card”,
    /// “internal_passport”, “utility_bill”, “bank_statement”,
    /// “rental_agreement”, “passport_registration”,
    /// “temporary_registration”
    pub kind: String, // TODO: rename to type
    /// Base64-encoded file hash
    pub file_hash: String,
    /// Error message
    pub message: String,
}

/// Represents an issue with the translated version of a document.
/// The error is considered resolved when a file
/// with the document translation change.
#[derive(Debug)]
pub struct PassportElementErrorTranslationFiles {
    /// Error source, must be translation_files
    pub source: String,
    /// Type of element of the user's Telegram Passport
    /// which has the issue, one of
    /// “passport”, “driver_license”, “identity_card”,
    /// “internal_passport”, “utility_bill”, “bank_statement”,
    /// “rental_agreement”, “passport_registration”,
    /// “temporary_registration”
    pub kind: String, // TODO: rename to type
    /// List of base64-encoded file hashes
    pub file_hashes: Vec<String>,
    /// Error message
    pub message: String,
}

/// Represents an issue in an unspecified place.
/// The error is considered resolved when new data is added.
#[derive(Debug)]
pub struct PassportElementErrorUnspecified {
    /// Error source, must be unspecified
    pub source: String,
    /// Type of element of the user's Telegram Passport which has the issue
    pub kind: String,
    /// TODO: rename to type
    /// Base64-encoded element hash
    pub element_hash: String,
    /// Error message
    pub message: String,
}
