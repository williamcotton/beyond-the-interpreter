use rlox_syntax::parser::parse;

fn main() {
    for source in [" 123 // note\n", "123 456", "", "é"] {
        match parse(source) {
            Ok(expr) => println!("{source:?} => {}", expr.to_prefix()),
            Err(error) => println!(
                "{source:?} => {} [{}..{})",
                error.message,
                error.range.start().0,
                error.range.end().0,
            ),
        }
    }
}
