use config::{QueensConfig, Config};

#[test]
fn is_valid() {
    let mut config = QueensConfig::new();

    assert!(config.is_valid(), "starting config is valid");

    config.map[0][0] = true;
    assert!(!config.is_valid(), "conflicting queens is invalid");

    config.map[0][0] = false;
    config.map[3][1] = true;
    assert!(config.is_valid(), "non-conflicting queens is valid");
}
