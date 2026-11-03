fn main() {
    let filename = std::env::args().nth(1).expect("expected a file");
    let source = std::fs::read_to_string(&filename).expect("could not read source file");
    print!("{source}");
}
