use std::fs::read_to_string;

use regex::Regex;
use toml::Value;

fn get_crate_version() -> String {
    let manifest = read_to_string("./Cargo.toml").expect("Failed to get Cargo.toml data");
    let value: Value = manifest.parse().expect("Failed to parse Cargo.toml");
    let version = value["package"]["version"]
        .as_str()
        .expect("Can not get version from Cargo.toml");
    String::from(version)
}

#[test]
fn versions() {
    let version = get_crate_version();
    for filename in &["./README.md"] {
        let readme = read_to_string(filename).unwrap();
        for pattern in &[
            r#"https://github\.com/tg-rs/carapax/tree/([\d\.]+)"#,
            r#"carapax\s?=\s?"([\d\.]+)""#,
            r#"carapax\s?=\s?\{\s?version\s?=\s?"([\d\.]+)""#,
        ] {
            let regex = Regex::new(pattern).expect("Can not create regex");
            for (line_idx, line_data) in readme.lines().enumerate() {
                let line_number = line_idx + 1;
                if let Some(captures) = regex.captures(line_data) {
                    let line_version = &captures[1];
                    assert_eq!(
                        line_version, version,
                        "Expects version {} at {}:{} '{}', found {}",
                        version, filename, line_number, line_data, line_version
                    );
                }
            }
        }
    }
}
