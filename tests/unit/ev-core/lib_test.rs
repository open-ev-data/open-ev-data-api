#[test]
fn test_public_exports_reachable() {
    // This test ensures that the main types are accessible from the crate root
    // which validates the `pub use` statements in lib.rs
    use ev_core::{Battery, Charging, ValidationError, Vehicle};

    // Just type checking existence
    #[allow(dead_code)]
    fn check_types(_v: Vehicle, _b: Battery, _c: Charging, _e: ValidationError) {}
}
