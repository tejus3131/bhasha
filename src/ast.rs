// src/ast.rs

#[derive(Debug, Clone)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    BinaryOp(Box<Expression>, BinOp, Box<Expression>),
    None
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Statement {
    Declaration(String, Expression),
    Assignment(String, Expression),
    If {
        condition: Expression,
        then_block: Vec<Statement>,
        else_block: Vec<Statement>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    Print(Expression),
    Input(String, String),
    FunctionDef (String, Vec<Expression>, Vec<Statement>, Expression ),
    FunctionCall (String, Vec<Expression>, String )
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}
