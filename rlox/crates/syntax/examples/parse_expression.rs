use rlox_syntax::parser::{ParseFailure, parse};

fn main() {
    for source in [
        " 123 // note\n",
        "123 456",
        "",
        "é",
        "(123)",
        "((12.5))",
        "( // é\n123 )",
        "()",
        "(123",
        "(123 456)",
        "123)",
        "-123",
        "--123",
        "-(123)",
        "(-123)",
        "-",
        "(-)",
        "-é",
        "8 / 4 / 2",
        "8 * 4 / 2",
        "8 / (4 / 2)",
        "-8 * 4 / 2",
        "8 *",
        "8 **2",
        "8 / é",
        "1 + 2 * 3",
        "(1 + 2) * 3",
        "8 - 3 - 1",
        "1--2",
        "-(1 + 2) * 3",
        "1 +",
        "1 + +2",
        "1 + é",
        "(1 + 2",
        " 001.0 // first\n + 02.00 * 3",
    ] {
        match parse(source) {
            Ok(expr) => println!("{source:?} => {}", expr.to_prefix()),
            Err(ParseFailure::Syntax(error)) => println!(
                "{source:?} => {} [{}..{})",
                error.message,
                usize::from(error.range.start()),
                usize::from(error.range.end()),
            ),
            Err(ParseFailure::SourceTooLarge(error)) => println!("{source:?} => {error}"),
        }
    }
}
