#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Grouping(Box<Expr>),
    Unary {
        op: UnaryOp,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum UnaryOp {
    Negate,
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Expr {
    pub fn to_prefix(&self) -> String {
        match self {
            Self::Number(value) => value.to_string(),
            Self::Grouping(child) => format!("(group {})", child.to_prefix()),
            Self::Unary { op, right } => {
                let symbol = match op {
                    UnaryOp::Negate => "-",
                };
                format!("({symbol} {})", right.to_prefix())
            }
            Self::Binary { left, op, right } => {
                let symbol = match op {
                    BinaryOp::Add => "+",
                    BinaryOp::Subtract => "-",
                    BinaryOp::Multiply => "*",
                    BinaryOp::Divide => "/",
                };
                format!("({symbol} {} {})", left.to_prefix(), right.to_prefix())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BinaryOp, Expr, UnaryOp};

    #[test]
    fn prints_numbers_and_nested_groups() {
        assert_eq!(Expr::Number(12.5).to_prefix(), "12.5");

        let nested = Expr::Grouping(Box::new(Expr::Grouping(Box::new(Expr::Number(123.0)))));
        assert_eq!(nested.to_prefix(), "(group (group 123))");
    }

    #[test]
    fn prints_unary_negation() {
        let expr = Expr::Unary {
            op: UnaryOp::Negate,
            right: Box::new(Expr::Number(123.0)),
        };
        assert_eq!(expr.to_prefix(), "(- 123)");
    }

    #[test]
    fn prints_binary_symbols_and_operand_order() {
        for (op, expected) in [
            (BinaryOp::Add, "(+ 8 2)"),
            (BinaryOp::Subtract, "(- 8 2)"),
            (BinaryOp::Multiply, "(* 8 2)"),
            (BinaryOp::Divide, "(/ 8 2)"),
        ] {
            let expr = Expr::Binary {
                left: Box::new(Expr::Number(8.0)),
                op,
                right: Box::new(Expr::Number(2.0)),
            };
            assert_eq!(expr.to_prefix(), expected);
        }
    }

    #[test]
    fn printing_preserves_the_mixed_tree() {
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

        let expected = "(* (- (group (+ 1 2))) 3)";
        assert_eq!(product.to_prefix(), expected);
        assert_eq!(product.to_prefix(), expected);
    }
}
