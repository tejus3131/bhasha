// src/ast.rs

#[derive(Debug)]
pub enum Expr {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    BinaryOp(Box<Expr>, BinOp, Box<Expr>),
    None
}

#[derive(Debug)]
pub enum BinOp {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Equals,
    NotEquals,
    And,
    Or,
}

#[derive(Debug)]
pub enum Statement {
    Assignment(String, Expr),
    If {
        condition: Expr,
        then_block: Vec<Statement>,
        else_block: Vec<Statement>,
    },
    While {
        condition: Expr,
        body: Vec<Statement>,
    },
    Print(Expr),
    Input(String, String),
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}
