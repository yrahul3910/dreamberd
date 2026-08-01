use std::fmt::Display;

use crate::tokens::TokenType;

/// A general expression in DreamBerd. This is essentially the AST.
pub enum Expression {
    Unary {
        /// Allowed: [`TokenType::Semicolon`], [`TokenType::Minus`],
        /// [`TokenType::IncrementOp`], [`TokenType::DecrementOp`]
        op: TokenType,
        expr: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        /// Allowed: [`TokenType::Plus`], [`TokenType::Minus`], [`TokenType::Asterisk`],
        /// [`TokenType::ForwardSlash`], [`TokenType::Caret`], [`TokenType::Caret`]
        op: Vec<TokenType>,
        right: Box<Expression>,
    },
    Literal(TokenType),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Unary { op, expr } => write!(f, "({op} {expr})"),
            Expression::Literal(tt) => write!(f, "{tt}"),
            Expression::Binary { left, op, right } => {
                write!(f, "BinOp(left={{{left}}}, op={{")?;
                for val in op {
                    write!(f, "{val}")?;
                }
                write!(f, "}}, right={{{right}}})")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::Expression, tokens::TokenType};

    #[test]
    fn test_unary_op() {
        let expr = Expression::Unary {
            op: TokenType::Minus,
            expr: Box::new(Expression::Literal(TokenType::Identifier("foo".into()))),
        };

        let displayed = expr.to_string();
        assert_eq!(displayed, "(- IDENTIFIER(foo))");
    }
}
