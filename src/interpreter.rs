use crate::ast::{BinOp, Expr, Program, Statement};

use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    None,
}

pub struct Interpreter {
    env: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env: HashMap::new(),
        }
    }

    fn eval_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Integer(i) => Value::Integer(*i),
            Expr::Float(f) => Value::Float(*f),
            Expr::String(s) => Value::String(s.clone()),
            Expr::Boolean(b) => Value::Boolean(*b),
            Expr::Identifier(name) => self.env.get(name).cloned().unwrap_or(Value::None),
            Expr::BinaryOp(lhs, op, rhs) => {
                let left = self.eval_expr(lhs);
                let right = self.eval_expr(rhs);
                match (left, right, op) {
                    (Value::Integer(l), Value::Integer(r), BinOp::Plus) => Value::Integer(l + r),
                    (Value::Float(l), Value::Float(r), BinOp::Plus) => Value::Float(l + r),
                    (Value::String(l), Value::String(r), BinOp::Plus) => {
                        Value::String(format!("{}{}", l, r))
                    }
                    (Value::String(l), _, BinOp::Plus) => {
                        Value::String(format!("{}{}", l, stringify!(r)))
                    }
                    (_, Value::String(r), BinOp::Plus) => {
                        Value::String(format!("{}{}", stringify!(l), r))
                    }
                    (Value::Integer(l), Value::Float(r), BinOp::Plus) => Value::Float(l as f64 + r),
                    (Value::Float(l), Value::Integer(r), BinOp::Plus) => Value::Float(l + r as f64),
                    (Value::Integer(l), Value::Integer(r), BinOp::Minus) => Value::Integer(l - r),
                    (Value::Float(l), Value::Float(r), BinOp::Minus) => Value::Float(l - r),
                    (Value::Integer(l), Value::Integer(r), BinOp::Multiply) => {
                        Value::Integer(l * r)
                    }
                    (Value::Float(l), Value::Float(r), BinOp::Multiply) => Value::Float(l * r),
                    (Value::Integer(l), Value::Integer(r), BinOp::Divide) => Value::Integer(l / r),
                    (Value::Float(l), Value::Float(r), BinOp::Divide) => Value::Float(l / r),
                    (Value::Integer(l), Value::Integer(r), BinOp::Modulo) => Value::Integer(l % r),
                    (Value::Float(l), Value::Float(r), BinOp::Modulo) => Value::Float(l % r),
                    (Value::Integer(l), Value::Integer(r), BinOp::LessThan) => {
                        Value::Boolean(l < r)
                    }
                    (Value::Float(l), Value::Float(r), BinOp::LessThan) => Value::Boolean(l < r),
                    (Value::Integer(l), Value::Integer(r), BinOp::GreaterThan) => {
                        Value::Boolean(l > r)
                    }
                    (Value::Float(l), Value::Float(r), BinOp::GreaterThan) => Value::Boolean(l > r),
                    (Value::Integer(l), Value::Integer(r), BinOp::LessThanOrEqual) => {
                        Value::Boolean(l <= r)
                    }
                    (Value::Float(l), Value::Float(r), BinOp::LessThanOrEqual) => {
                        Value::Boolean(l <= r)
                    }
                    (Value::Integer(l), Value::Integer(r), BinOp::GreaterThanOrEqual) => {
                        Value::Boolean(l >= r)
                    }
                    (Value::Float(l), Value::Float(r), BinOp::GreaterThanOrEqual) => {
                        Value::Boolean(l >= r)
                    }
                    (Value::Integer(l), Value::Integer(r), BinOp::Equals) => Value::Boolean(l == r),
                    (Value::Float(l), Value::Float(r), BinOp::Equals) => Value::Boolean(l == r),
                    (Value::String(l), Value::String(r), BinOp::Equals) => Value::Boolean(l == r),
                    (Value::Boolean(l), Value::Boolean(r), BinOp::Equals) => Value::Boolean(l == r),
                    (Value::Integer(l), Value::Integer(r), BinOp::NotEquals) => {
                        Value::Boolean(l != r)
                    }
                    (Value::Float(l), Value::Float(r), BinOp::NotEquals) => Value::Boolean(l != r),
                    (Value::String(l), Value::String(r), BinOp::NotEquals) => {
                        Value::Boolean(l != r)
                    }
                    (Value::Boolean(l), Value::Boolean(r), BinOp::NotEquals) => {
                        Value::Boolean(l != r)
                    }
                    (Value::Boolean(l), Value::Boolean(r), BinOp::And) => Value::Boolean(l && r),
                    (Value::Boolean(l), Value::Boolean(r), BinOp::Or) => Value::Boolean(l || r),
                    _ => panic!("Invalid operation"),
                }
            }
            Expr::None => panic!("Invalid Token"),
        }
    }

    fn exec_stmt(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Assignment(name, expr) => {
                let value = self.eval_expr(expr);
                self.env.insert(name.clone(), value);
            }
            Statement::If {
                condition,
                then_block,
                else_block,
            } => {
                let cond = self.eval_expr(condition);
                if self.is_truthy(&cond) {
                    for stmt in then_block {
                        self.exec_stmt(stmt);
                    }
                } else {
                    for stmt in else_block {
                        self.exec_stmt(stmt);
                    }
                }
            }
            Statement::While { condition, body } => {
                let mut eval = self.eval_expr(condition);
                while self.is_truthy(&eval) {
                    for stmt in body {
                        self.exec_stmt(stmt);
                    }
                    eval = self.eval_expr(condition);
                }
            }
            Statement::Print(expr) => {
                let value = self.eval_expr(expr);
                let data = match value {
                    Value::String(x) => x.to_string(),
                    Value::Boolean(x) => x.to_string(),
                    Value::Float(x) => x.to_string(),
                    Value::Integer(x) => x.to_string(),
                    Value::None => "khali".to_string(),
                };
                println!("{}", data);
            }
            Statement::Input(dtype, name) => {
                print!("{} ({}) >>> ", dtype, name);
                io::stdout().flush().unwrap_or_else(|data| {
                    println!("{}", data);
                    std::process::exit(1);
                });
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        let value = input.trim();

                        let value = if dtype == "sankhya" {
                            Value::Integer(value.parse().unwrap_or_else(|data| {
                                println!("{}", data);
                                std::process::exit(1);
                            }))
                        } else if dtype == "dasamlav" {
                            Value::Float(value.parse().unwrap_or_else(|data| {
                                println!("{}", data);
                                std::process::exit(1);
                            }))
                        } else if dtype == "paath" {
                            Value::String(value.to_string())
                        } else if dtype == "tark" {
                            Value::Boolean(value.parse().unwrap_or_else(|data| {
                                println!("{}", data);
                                std::process::exit(1);
                            }))
                        } else {
                            println!("{}", dtype);
                            panic!("Invalid data type")
                        };

                        self.env.insert(name.clone(), value);
                    }
                    Err(error) => {
                        eprintln!("Error reading input: {}", error);
                    }
                }
            }
        }
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Boolean(b) => *b,
            Value::Integer(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::None => false,
        }
    }

    pub fn run(&mut self, program: Program) {
        for stmt in program.statements {
            // println!("{:?}", stmt);
            self.exec_stmt(&stmt);
        }
    }
}
