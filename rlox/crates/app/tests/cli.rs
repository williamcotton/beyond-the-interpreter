use std::fs;
use std::process::Command;

use tempfile::tempdir;

#[test]
fn tokens_show_utf8_ranges_and_trivia() {
    let directory = tempdir().expect("create test directory");
    let input = directory.path().join("input.lox");
    fs::write(&input, "print \"é\";\n").expect("write test source");

    let output = Command::new(env!("CARGO_BIN_EXE_rlox"))
        .arg("tokens")
        .arg(&input)
        .output()
        .expect("run rlox");

    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        concat!(
            "Print [0..5) \"print\"\n",
            "Whitespace [5..6) \" \"\n",
            "String [6..10) \"\\\"é\\\"\"\n",
            "Semicolon [10..11) \";\"\n",
            "Whitespace [11..12) \"\\n\"\n",
        )
        .as_bytes()
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn lexical_errors_keep_the_listing() {
    let directory = tempdir().expect("create test directory");
    let input = directory.path().join("input.lox");

    for (source, expected) in [
        (
            "@é!\n",
            concat!(
                "Error [0..1) \"@\"\n",
                "Error [1..3) \"é\"\n",
                "Bang [3..4) \"!\"\n",
                "Whitespace [4..5) \"\\n\"\n",
            ),
        ),
        ("\"é\n", "Error [0..4) \"\\\"é\\n\"\n"),
    ] {
        fs::write(&input, source).expect("write test source");
        let output = Command::new(env!("CARGO_BIN_EXE_rlox"))
            .arg("tokens")
            .arg(&input)
            .output()
            .expect("run rlox");

        assert_eq!(output.status.code(), Some(1));
        assert_eq!(output.stdout, expected.as_bytes());
        assert_eq!(output.stderr, b"rlox: scan contains lexical errors\n");
    }
}

#[test]
fn empty_input_succeeds_without_output() {
    let directory = tempdir().expect("create test directory");
    let input = directory.path().join("empty.lox");
    fs::write(&input, "").expect("write empty source");

    let output = Command::new(env!("CARGO_BIN_EXE_rlox"))
        .arg("tokens")
        .arg(&input)
        .output()
        .expect("run rlox");

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}

#[test]
fn source_still_displays_malformed_lox() {
    let directory = tempdir().expect("create test directory");
    let input = directory.path().join("input.lox");
    let source = "@é!\n";
    fs::write(&input, source).expect("write test source");

    let output = Command::new(env!("CARGO_BIN_EXE_rlox"))
        .arg("source")
        .arg(&input)
        .output()
        .expect("run rlox");

    assert!(output.status.success());
    assert_eq!(output.stdout, source.as_bytes());
    assert!(output.stderr.is_empty());
}

#[test]
fn missing_file_reports_a_read_error() {
    let directory = tempdir().expect("create test directory");
    let missing = directory.path().join("missing.lox");

    for command in ["source", "tokens"] {
        let output = Command::new(env!("CARGO_BIN_EXE_rlox"))
            .arg(command)
            .arg(&missing)
            .output()
            .expect("run rlox");

        assert_eq!(output.status.code(), Some(1));
        assert!(output.stdout.is_empty());
        let stderr = String::from_utf8(output.stderr).expect("UTF-8 error message");
        assert!(stderr.starts_with("rlox: could not read '"));
        assert!(stderr.contains("missing.lox"));
    }
}

#[test]
fn tokens_requires_a_filename() {
    let output = Command::new(env!("CARGO_BIN_EXE_rlox"))
        .arg("tokens")
        .output()
        .expect("run rlox");

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).expect("UTF-8 usage message");
    assert!(stderr.contains("Usage: rlox tokens <FILE>"));
}
