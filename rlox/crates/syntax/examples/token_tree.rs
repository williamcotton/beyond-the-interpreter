use rlox_syntax::tree::token_tree;

fn main() {
    for source in ["001.50", " // é\n12.5 // end", "@é!"] {
        match token_tree(source) {
            Ok(root) => {
                println!("source: {source:?}");
                println!(
                    "root: {:?} [{}..{})",
                    root.kind(),
                    usize::from(root.text_range().start()),
                    usize::from(root.text_range().end())
                );
                for element in root.children_with_tokens() {
                    println!(
                        "  {:?} [{}..{}) {:?}",
                        element.kind(),
                        usize::from(element.text_range().start()),
                        usize::from(element.text_range().end()),
                        element.to_string()
                    );
                }
                println!("reconstructed: {:?}", root.to_string());
            }
            Err(error) => eprintln!("{error}"),
        }
    }
}
