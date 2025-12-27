use clap::Parser;
use ev_etl::cli::Cli;

#[test]
fn test_cli_parsing_defaults() {
    let args = vec!["ev-etl", "--input", "./data"];
    let cli = Cli::parse_from(args);

    assert_eq!(cli.input.to_str().unwrap(), "./data");
    assert_eq!(cli.output.to_str().unwrap(), "./output");
    assert_eq!(cli.formats, vec!["json", "sqlite"]);
    assert!(!cli.validate_only);
    assert!(!cli.verbose);
}

#[test]
fn test_cli_parsing_custom() {
    let args = vec![
        "ev-etl",
        "--input",
        "./in",
        "--output",
        "./out",
        "--formats",
        "csv,xml",
        "--validate-only",
        "--verbose",
    ];
    let cli = Cli::parse_from(args);

    assert_eq!(cli.input.to_str().unwrap(), "./in");
    assert_eq!(cli.output.to_str().unwrap(), "./out");
    assert_eq!(cli.formats, vec!["csv", "xml"]);
    assert!(cli.validate_only);
    assert!(cli.verbose);
}
