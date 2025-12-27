use ev_server::Config;

#[test]
fn test_config_defaults() {
    // We can't safely manipulate env vars in parallel tests without serial_test,
    // but we can at least check that from_env returns a valid config or error.
    // If env vars are not set, it uses defaults.

    // Ensure clean state (best effort)
    // std::env::remove_var("PORT");

    let config = Config::from_env();
    assert!(config.is_ok());
    let config = config.unwrap();

    assert_eq!(config.host, "0.0.0.0");
    assert_eq!(config.log_level, "info");
}
