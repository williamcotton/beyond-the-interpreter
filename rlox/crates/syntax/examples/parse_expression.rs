use rlox_syntax::{ast::Expr, cst::parse};

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
            Ok(parsed) => {
                if parsed.errors.is_empty() {
                    let prefix = parsed
                        .root
                        .first_child()
                        .and_then(Expr::cast)
                        .and_then(|expr| expr.to_prefix());
                    println!(
                        "{source:?} => {}",
                        prefix.as_deref().unwrap_or("<incomplete>")
                    );
                } else {
                    for error in parsed.errors {
                        println!(
                            "{source:?} => {} [{}..{})",
                            error.message,
                            usize::from(error.range.start()),
                            usize::from(error.range.end())
                        );
                    }
                }
            }
            Err(error) => println!("{source:?} => {error}"),
        }
    }
}
