use crate::types::passport::*;

#[test]
fn test_deserialize_data() {
    let input = r#"{
        "data": [
            {
                "type": "address",
                "data": "d",
                "hash": "h"
            },
            {
                "type": "bank_statement",
                "files": [{"file_id": "f", "file_size": 1, "file_date": 0}],
                "translation": [{"file_id": "f", "file_size": 1, "file_date": 0}],
                "hash": "h"
            },
            {
                "type": "driver_license",
                "data": "d",
                "front_side": {"file_id": "f", "file_size": 1, "file_date": 0},
                "reverse_side": {"file_id": "f", "file_size": 1, "file_date": 0},
                "selfie": {"file_id": "f", "file_size": 1, "file_date": 0},
                "translation": [{"file_id": "f", "file_size": 1, "file_date": 0}],
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
                "front_side": {"file_id": "f", "file_size": 1, "file_date": 0},
                "reverse_side": {"file_id": "f", "file_size": 1, "file_date": 0},
                "selfie": {"file_id": "f", "file_size": 1, "file_date": 0},
                "translation": [{"file_id": "f", "file_size": 1, "file_date": 0}],
                "hash": "h"
            },
            {
                "type": "internal_passport",
                "data": "d",
                "front_side": {"file_id": "f", "file_size": 1, "file_date": 0},
                "selfie": {"file_id": "f", "file_size": 1, "file_date": 0},
                "translation": [{"file_id": "f", "file_size": 1, "file_date": 0}],
                "hash": "h"
            },
            {
                "type": "passport",
                "data": "d",
                "front_side": {"file_id": "f", "file_size": 1, "file_date": 0},
                "selfie": {"file_id": "f", "file_size": 1, "file_date": 0},
                "translation": [{"file_id": "f", "file_size": 1, "file_date": 0}],
                "hash": "h"
            },
            {
                "type": "passport_registration",
                "files": [{"file_id": "f", "file_size": 1, "file_date": 0}],
                "translation": [{"file_id": "f", "file_size": 1, "file_date": 0}],
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
                "files": [{"file_id": "f", "file_size": 1, "file_date": 0}],
                "translation": [{"file_id": "f", "file_size": 1, "file_date": 0}],
                "hash": "h"
            },
            {
                "type": "temporary_registration",
                "files": [{"file_id": "f", "file_size": 1, "file_date": 0}],
                "translation": [{"file_id": "f", "file_size": 1, "file_date": 0}],
                "hash": "h"
            },
            {
                "type": "utility_bill",
                "files": [{"file_id": "f", "file_size": 1, "file_date": 0}],
                "translation": [{"file_id": "f", "file_size": 1, "file_date": 0}],
                "hash": "h"
            }
        ],
        "credentials": {
            "data": "d",
            "hash": "h",
            "secret": "s"
        }
    }"#;
    let data: PassportData = serde_json::from_str(input).unwrap();
    assert_eq!(data.credentials.data, String::from("d"));
    assert_eq!(data.credentials.hash, String::from("h"));
    assert_eq!(data.credentials.secret, String::from("s"));
    assert_eq!(data.data.len(), 13);
}

#[test]
fn test_serialize_error() {
    let err = PassportElementError::data_field(
        EncryptedPassportElementKind::Address,
        "address",
        "data_hash",
        "bad address",
    )
    .unwrap();
    let j = serde_json::to_string(&err).unwrap();
    assert_eq!(j, r#"{"source":"data","type":"address","field_name":"address","data_hash":"data_hash","message":"bad address"}"#);

    let err = PassportElementError::front_side(
        EncryptedPassportElementKind::DriverLicense,
        "file_hash",
        "bad file",
    )
    .unwrap();
    let j = serde_json::to_string(&err).unwrap();
    assert_eq!(j, r#"{"source":"front_side","type":"driver_license","file_hash":"file_hash","message":"bad file"}"#);

    let err = PassportElementError::reverse_side(
        EncryptedPassportElementKind::DriverLicense,
        "file_hash",
        "bad file",
    )
    .unwrap();
    let j = serde_json::to_string(&err).unwrap();
    assert_eq!(j, r#"{"source":"reverse_side","type":"driver_license","file_hash":"file_hash","message":"bad file"}"#);

    let err = PassportElementError::selfie(
        EncryptedPassportElementKind::DriverLicense,
        "file_hash",
        "bad file",
    )
    .unwrap();
    let j = serde_json::to_string(&err).unwrap();
    assert_eq!(j, r#"{"source":"selfie","type":"driver_license","file_hash":"file_hash","message":"bad file"}"#);

    let err = PassportElementError::file(
        EncryptedPassportElementKind::BankStatement,
        "file_hash",
        "bad file",
    )
    .unwrap();
    let j = serde_json::to_string(&err).unwrap();
    assert_eq!(
        j,
        r#"{"source":"file","type":"bank_statement","file_hash":"file_hash","message":"bad file"}"#
    );

    let err = PassportElementError::files(
        EncryptedPassportElementKind::BankStatement,
        vec![String::from("file_hash")],
        "bad file",
    )
    .unwrap();
    let j = serde_json::to_string(&err).unwrap();
    assert_eq!(j, r#"{"source":"files","type":"bank_statement","file_hashes":["file_hash"],"message":"bad file"}"#);

    let err = PassportElementError::translation_file(
        EncryptedPassportElementKind::BankStatement,
        "file_hash",
        "bad file",
    )
    .unwrap();
    let j = serde_json::to_string(&err).unwrap();
    assert_eq!(
        j,
        r#"{"source":"translation_file","type":"bank_statement","file_hash":"file_hash","message":"bad file"}"#
    );

    let err = PassportElementError::translation_files(
        EncryptedPassportElementKind::BankStatement,
        vec![String::from("file_hash")],
        "bad file",
    )
    .unwrap();
    let j = serde_json::to_string(&err).unwrap();
    assert_eq!(j, r#"{"source":"translation_files","type":"bank_statement","file_hashes":["file_hash"],"message":"bad file"}"#);

    let err = PassportElementError::unspecified(
        EncryptedPassportElementKind::BankStatement,
        "element_hash",
        "bad file",
    );
    let j = serde_json::to_string(&err).unwrap();
    assert_eq!(
        j,
        r#"{"source":"unspecified","type":"bank_statement","element_hash":"element_hash","message":"bad file"}"#
    );
}

#[test]
fn test_create_error_accepts_kind() {
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
        let err = PassportElementError::translation_files(
            kind,
            vec![String::from("file_hash")],
            "bad file",
        );
        assert!(if flag { err.is_ok() } else { err.is_err() });
    }
}
