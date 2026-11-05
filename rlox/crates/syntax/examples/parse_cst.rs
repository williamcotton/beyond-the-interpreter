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
    ] {
        match parse(source) {
            Ok(parsed) => {
                println!("source: {source:?}");
                for child in parsed.root.children_with_tokens() {
                    println!(
                        "  {:?} [{}..{}) {:?}",
                        child.kind(),
                        usize::from(child.text_range().start()),
                        usize::from(child.text_range().end()),
                        child.to_string()
                    );
                }
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
