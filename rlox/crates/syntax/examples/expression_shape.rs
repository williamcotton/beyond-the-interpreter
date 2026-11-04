use rlox_syntax::expr::Expr;

fn main() {
    let number = Expr::Number(123.0);
    println!("number: {number:?}");

    let grouped = Expr::Grouping(Box::new(number));
    println!("grouped: {grouped:?}");

    let nested = Expr::Grouping(Box::new(grouped));
    println!("nested: {nested:?}");
}
