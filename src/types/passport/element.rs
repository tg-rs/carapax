use crate::types::passport::PassportFile;
use serde::de::{Deserialize, Deserializer, Error};

/// Contains information about documents
/// or other Telegram Passport elements
/// shared with the bot by the user
#[derive(Debug)]
pub enum EncryptedPassportElement {
    /// Address
    Address(EncryptedPassportElementAddress),
    /// Bank statement
    BankStatement(EncryptedPassportElementBankStatement),
    /// Driver license
    DriverLicense(EncryptedPassportElementDriverLicense),
    /// E-Mail
    Email(EncryptedPassportElementEmail),
    /// Identity card
    IdentityCard(EncryptedPassportElementIdentityCard),
    /// Internal passport
    InternalPassport(EncryptedPassportElementInternalPassport),
    /// Passport
    Passport(EncryptedPassportElementPassport),
    /// Passport registration
    PassportRegistration(EncryptedPassportElementPassportRegistration),
    /// Personal details
    PersonalDetails(EncryptedPassportElementPersonalDetails),
    /// Phone number
    PhoneNumber(EncryptedPassportElementPhoneNumber),
    /// Rental agreement
    RentalAgreement(EncryptedPassportElementRentalAgreement),
    /// Temporary registration
    TemporaryRegistration(EncryptedPassportElementTemporaryRegistration),
    /// Utility bill
    UtilityBill(EncryptedPassportElementUtilityBill),
}

impl<'de> Deserialize<'de> for EncryptedPassportElement {
    fn deserialize<D>(deserializer: D) -> Result<EncryptedPassportElement, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: RawEncryptedPassportElement = Deserialize::deserialize(deserializer)?;
        use self::EncryptedPassportElementKind::*;
        macro_rules! required {
            ($name:ident) => {{
                match raw.$name {
                    Some(val) => val,
                    None => return Err(D::Error::missing_field(stringify!($name))),
                }
            }};
        }
        Ok(match raw.kind {
            Address => EncryptedPassportElement::Address(EncryptedPassportElementAddress {
                data: required!(data),
                hash: raw.hash,
            }),
            BankStatement => {
                EncryptedPassportElement::BankStatement(EncryptedPassportElementBankStatement {
                    files: required!(files),
                    translation: raw.translation,
                    hash: raw.hash,
                })
            }
            DriverLicense => {
                EncryptedPassportElement::DriverLicense(EncryptedPassportElementDriverLicense {
                    data: required!(data),
                    front_side: required!(front_side),
                    reverse_side: required!(reverse_side),
                    selfie: required!(selfie),
                    translation: raw.translation,
                    hash: raw.hash,
                })
            }
            Email => EncryptedPassportElement::Email(EncryptedPassportElementEmail {
                email: required!(email),
                hash: raw.hash,
            }),
            IdentityCard => {
                EncryptedPassportElement::IdentityCard(EncryptedPassportElementIdentityCard {
                    data: required!(data),
                    front_side: required!(front_side),
                    reverse_side: required!(reverse_side),
                    selfie: required!(selfie),
                    translation: raw.translation,
                    hash: raw.hash,
                })
            }
            InternalPassport => EncryptedPassportElement::InternalPassport(
                EncryptedPassportElementInternalPassport {
                    data: required!(data),
                    front_side: required!(front_side),
                    selfie: required!(selfie),
                    translation: raw.translation,
                    hash: raw.hash,
                },
            ),
            Passport => EncryptedPassportElement::Passport(EncryptedPassportElementPassport {
                data: required!(data),
                front_side: required!(front_side),
                selfie: required!(selfie),
                translation: raw.translation,
                hash: raw.hash,
            }),
            PassportRegistration => EncryptedPassportElement::PassportRegistration(
                EncryptedPassportElementPassportRegistration {
                    files: required!(files),
                    translation: raw.translation,
                    hash: raw.hash,
                },
            ),
            PersonalDetails => {
                EncryptedPassportElement::PersonalDetails(EncryptedPassportElementPersonalDetails {
                    data: required!(data),
                    hash: raw.hash,
                })
            }
            PhoneNumber => {
                EncryptedPassportElement::PhoneNumber(EncryptedPassportElementPhoneNumber {
                    phone_number: required!(phone_number),
                    hash: raw.hash,
                })
            }
            RentalAgreement => {
                EncryptedPassportElement::RentalAgreement(EncryptedPassportElementRentalAgreement {
                    files: required!(files),
                    translation: raw.translation,
                    hash: raw.hash,
                })
            }
            TemporaryRegistration => EncryptedPassportElement::TemporaryRegistration(
                EncryptedPassportElementTemporaryRegistration {
                    files: required!(files),
                    translation: raw.translation,
                    hash: raw.hash,
                },
            ),
            UtilityBill => {
                EncryptedPassportElement::UtilityBill(EncryptedPassportElementUtilityBill {
                    files: required!(files),
                    translation: raw.translation,
                    hash: raw.hash,
                })
            }
        })
    }
}

/// Address
#[derive(Debug)]
pub struct EncryptedPassportElementAddress {
    /// Base64-encoded encrypted
    /// Telegram Passport element data provided by the user
    /// Can be decrypted and verified using
    /// the accompanying EncryptedCredentials
    pub data: String,
    /// Base64-encoded element hash for
    /// using in PassportElementErrorUnspecified
    pub hash: String,
}

/// Bank statement
#[derive(Debug)]
pub struct EncryptedPassportElementBankStatement {
    /// Array of encrypted files with
    /// documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub files: Vec<PassportFile>,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub translation: Option<Vec<PassportFile>>,
    /// Base64-encoded element hash for
    /// using in PassportElementErrorUnspecified
    pub hash: String,
}

/// Driver license
#[derive(Debug)]
pub struct EncryptedPassportElementDriverLicense {
    /// Base64-encoded encrypted
    /// Telegram Passport element data provided by the user
    /// Can be decrypted and verified using
    /// the accompanying EncryptedCredentials
    pub data: String,
    /// Encrypted file with the front side
    /// of the document, provided by the user
    /// The file can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub front_side: PassportFile,
    /// Encrypted file with the reverse side of the document,
    /// provided by the user
    /// The file can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub reverse_side: PassportFile,
    /// Encrypted file with the selfie of the user
    /// holding a document, provided by the user
    /// The file can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub selfie: PassportFile,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub translation: Option<Vec<PassportFile>>,
    /// Base64-encoded element hash for
    /// using in PassportElementErrorUnspecified
    pub hash: String,
}

/// E-Mail
#[derive(Debug)]
pub struct EncryptedPassportElementEmail {
    /// User's verified email address
    pub email: String,
    /// Base64-encoded element hash for
    /// using in PassportElementErrorUnspecified
    pub hash: String,
}

/// Identity card
#[derive(Debug)]
pub struct EncryptedPassportElementIdentityCard {
    /// Base64-encoded encrypted
    /// Telegram Passport element data provided by the user
    /// Can be decrypted and verified using
    /// the accompanying EncryptedCredentials
    pub data: String,
    /// Encrypted file with the front side
    /// of the document, provided by the user
    /// The file can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub front_side: PassportFile,
    /// Encrypted file with the reverse side of the document,
    /// provided by the user
    /// The file can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub reverse_side: PassportFile,
    /// Encrypted file with the selfie of the user
    /// holding a document, provided by the user
    /// The file can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub selfie: PassportFile,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub translation: Option<Vec<PassportFile>>,
    /// Base64-encoded element hash for
    /// using in PassportElementErrorUnspecified
    pub hash: String,
}

/// Internal passport
#[derive(Debug)]
pub struct EncryptedPassportElementInternalPassport {
    /// Base64-encoded encrypted
    /// Telegram Passport element data provided by the user
    /// Can be decrypted and verified using
    /// the accompanying EncryptedCredentials
    pub data: String,
    /// Encrypted file with the front side
    /// of the document, provided by the user
    /// The file can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub front_side: PassportFile,
    /// Encrypted file with the selfie of the user
    /// holding a document, provided by the user
    /// The file can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub selfie: PassportFile,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub translation: Option<Vec<PassportFile>>,
    /// Base64-encoded element hash for
    /// using in PassportElementErrorUnspecified
    pub hash: String,
}

/// Passport
#[derive(Debug)]
pub struct EncryptedPassportElementPassport {
    /// Base64-encoded encrypted
    /// Telegram Passport element data provided by the user
    /// Can be decrypted and verified using
    /// the accompanying EncryptedCredentials
    pub data: String,
    /// Encrypted file with the front side
    /// of the document, provided by the user
    /// The file can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub front_side: PassportFile,
    /// Encrypted file with the selfie of the user
    /// holding a document, provided by the user
    /// The file can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub selfie: PassportFile,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub translation: Option<Vec<PassportFile>>,
    /// Base64-encoded element hash for
    /// using in PassportElementErrorUnspecified
    pub hash: String,
}

/// Passport registration
#[derive(Debug)]
pub struct EncryptedPassportElementPassportRegistration {
    /// Array of encrypted files with
    /// documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub files: Vec<PassportFile>,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub translation: Option<Vec<PassportFile>>,
    /// Base64-encoded element hash for
    /// using in PassportElementErrorUnspecified
    pub hash: String,
}

/// Personal details
#[derive(Debug)]
pub struct EncryptedPassportElementPersonalDetails {
    /// Base64-encoded encrypted
    /// Telegram Passport element data provided by the user
    /// Can be decrypted and verified using
    /// the accompanying EncryptedCredentials
    pub data: String,
    /// Base64-encoded element hash for
    /// using in PassportElementErrorUnspecified
    pub hash: String,
}

/// Phone number
#[derive(Debug)]
pub struct EncryptedPassportElementPhoneNumber {
    /// User's verified phone number
    pub phone_number: String,
    /// Base64-encoded element hash for
    /// using in PassportElementErrorUnspecified
    pub hash: String,
}

/// Rental agreement
#[derive(Debug)]
pub struct EncryptedPassportElementRentalAgreement {
    /// Array of encrypted files with
    /// documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub files: Vec<PassportFile>,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub translation: Option<Vec<PassportFile>>,
    /// Base64-encoded element hash for
    /// using in PassportElementErrorUnspecified
    pub hash: String,
}

/// Temporary registration
#[derive(Debug)]
pub struct EncryptedPassportElementTemporaryRegistration {
    /// Array of encrypted files with
    /// documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub files: Vec<PassportFile>,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub translation: Option<Vec<PassportFile>>,
    /// Base64-encoded element hash for
    /// using in PassportElementErrorUnspecified
    pub hash: String,
}

/// Utility bill
#[derive(Debug)]
pub struct EncryptedPassportElementUtilityBill {
    /// Array of encrypted files with
    /// documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub files: Vec<PassportFile>,
    /// Array of encrypted files with translated
    /// versions of documents provided by the user
    /// Files can be decrypted and verified
    /// using the accompanying EncryptedCredentials
    pub translation: Option<Vec<PassportFile>>,
    /// Base64-encoded element hash for
    /// using in PassportElementErrorUnspecified
    pub hash: String,
}

#[derive(Debug, Deserialize)]
struct RawEncryptedPassportElement {
    #[serde(rename = "type")]
    kind: EncryptedPassportElementKind,
    data: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    files: Option<Vec<PassportFile>>,
    front_side: Option<PassportFile>,
    reverse_side: Option<PassportFile>,
    selfie: Option<PassportFile>,
    translation: Option<Vec<PassportFile>>,
    hash: String,
}

/// Type of encrypted passport element
#[derive(Debug, Deserialize, Serialize)]
pub enum EncryptedPassportElementKind {
    /// Address
    #[serde(rename = "address")]
    Address,
    /// Bank statement
    #[serde(rename = "bank_statement")]
    BankStatement,
    /// Driver license
    #[serde(rename = "driver_license")]
    DriverLicense,
    /// E-Mail
    #[serde(rename = "email")]
    Email,
    /// Identity card
    #[serde(rename = "identity_card")]
    IdentityCard,
    /// Internal passport
    #[serde(rename = "internal_passport")]
    InternalPassport,
    /// Passport
    #[serde(rename = "passport")]
    Passport,
    /// Passport registration
    #[serde(rename = "passport_registration")]
    PassportRegistration,
    /// Personal details
    #[serde(rename = "personal_details")]
    PersonalDetails,
    /// Phone number
    #[serde(rename = "phone_number")]
    PhoneNumber,
    /// Rental agreement
    #[serde(rename = "rental_agreement")]
    RentalAgreement,
    /// Temporary registration
    #[serde(rename = "temporary_registration")]
    TemporaryRegistration,
    /// Utility bill
    #[serde(rename = "utility_bill")]
    UtilityBill,
}
