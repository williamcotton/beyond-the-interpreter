use rlox_syntax::expr::{BinaryOp, Expr, UnaryOp};

fn main() {
    let number = Expr::Number(123.0);
    println!("number: {number:?}");

    let grouped = Expr::Grouping(Box::new(number));
    println!("grouped: {grouped:?}");

    let nested = Expr::Grouping(Box::new(grouped));
    println!("nested: {nested:?}");

    let negated = Expr::Unary {
        op: UnaryOp::Negate,
        right: Box::new(Expr::Number(123.0)),
    };
    println!("negated: {negated:?}");

    let sum = Expr::Binary {
        left: Box::new(Expr::Number(1.0)),
        op: BinaryOp::Add,
        right: Box::new(Expr::Number(2.0)),
    };
    let grouped_sum = Expr::Grouping(Box::new(sum));
    let negative_sum = Expr::Unary {
        op: UnaryOp::Negate,
        right: Box::new(grouped_sum),
    };
    let product = Expr::Binary {
        left: Box::new(negative_sum),
        op: BinaryOp::Multiply,
        right: Box::new(Expr::Number(3.0)),
    };
    println!("arithmetic: {product:#?}");

    println!("prefix: {}", product.to_prefix());
    println!("prefix again: {}", product.to_prefix());
}
