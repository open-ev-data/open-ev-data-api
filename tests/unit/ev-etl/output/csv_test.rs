#[test]
fn test_csv_escaping() {
    let value = "Tesla, Inc.";
    let escaped = format!("\"{}\"", value.replace("\"", "\"\""));
    assert_eq!(escaped, "\"Tesla, Inc.\"");
}
