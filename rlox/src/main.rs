fn main() {
    let filename = std::env::args().nth(1).expect("expected a file");
    println!("{filename}");
}
