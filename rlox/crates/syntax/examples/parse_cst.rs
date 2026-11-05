use rlox_syntax::cst::parse;

fn main() {
    for source in [
        " ( 001.50 ) ",
        "(12.5",
        "(1 @)",
        "1 + 2",
        "- // é\n12.5",
        "--1",
        "-(001.50)",
        "(-1)",
        "-",
        "(-)",
        "-é",
        "8 / 4 / 2",
        "8 / (4 / 2)",
        "1 + 2 * 3",
        "(1 + 2) * 3",
        "8 - 3 - 1",
        "-(1 + 2) * 3",
        "1 +",
        "1 + é",
        " 001.0 // first\n + 02.00 * 3",
        "* 2",
        "(1 + )",
        "(1 @) + é",
        "1 ) é",
    ] {
        match parse(source) {
            Ok(parsed) => {
                println!("source: {source:?}");
                print!("{:#?}", parsed.root);
                for error in parsed.errors {
                    println!(
                        "error: {} [{}..{})",
                        error.message,
                        usize::from(error.range.start()),
                        usize::from(error.range.end())
                    );
                }
                println!("reconstructed: {:?}", parsed.root.to_string());
            }
            Err(error) => eprintln!("{error}"),
        }
    }
}
