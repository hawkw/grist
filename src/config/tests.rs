use super::parse_toml;
use std::path::PathBuf;

static TEST_TOML: &'static str = r#"
    [grist]
    roots = ['~/a', '/b/']
    port  = 3000

    [logging]
    debug = true
"#;

#[test]
fn test_toml_port() {
    let result = parse_toml(TEST_TOML).unwrap();
    assert_eq!(result.port, 3000);
}

#[test]
fn test_toml_debug() {
    let result = parse_toml(TEST_TOML).unwrap();
    assert_eq!(result.debug, true);
}

#[test]
fn test_toml_roots() {
    let result = parse_toml(TEST_TOML).unwrap();
    assert_eq!(result.roots, Some(vec!(PathBuf::from("~/a"), PathBuf::from("/b/"))));
}
