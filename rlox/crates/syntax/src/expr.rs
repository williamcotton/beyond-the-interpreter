#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Grouping(Box<Expr>),
}
