// src/parser.rs

use crate::ast::*;
use crate::tokens::Token;

pub struct Parser<'a> {
    lexer: &'a mut Vec<Token>,
    current_token: usize,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Vec<Token>) -> Self {
        Parser {
            lexer,
            current_token: 0,
        }
    }

    fn next_token(&mut self) {
        if self.has_next() {
            self.current_token = self.current_token + 1;
        }
    }

    fn previous_token(&mut self) {
        if self.current_token > 0 {
            self.current_token = self.current_token - 1;
        }
    }

    fn current_token(&self) -> Token {
        self.lexer[self.current_token].clone()
    }

    fn has_next(&self) -> bool {
        self.current_token() != Token::TheEnd
    }

    pub fn print_tokens(&mut self) {

        let current_state = self.current_token;
        self.current_token = 0;

        while self.has_next() {
            // println!("{:?}", self.current_token());
            self.next_token();
        }
        self.current_token = current_state;
    }
    
    pub fn parse(&mut self) -> Program {
        let mut statements = Vec::new();
        
        while self.has_next() {
            // println!("{:?}", self.current_token());
            if let Some(statement) = self.parse_statement() {
                // println!("{:?}", statement);
                statements.push(statement);
            }
        }

        Program { statements }
    }

    fn parse_function_def(&mut self) -> Statement {
        self.next_token(); // banao
        // println!("{:?}", self.current_token());
        let func_name = if let Token::Identifier(name) = self.current_token() {
            name.clone()
        } else {
            panic!("Expected identifier after 'banao'")
        };

        self.next_token();
        // println!("{:?}", self.current_token());

        self.next_token(); // jo le
        // println!("{:?}", self.current_token());

        let mut params = Vec::new();
        while self.current_token() != Token::ParamEnd {
            params.push(self.parse_primary());
            self.next_token(); 
            // println!("{:?}", self.current_token());
        }

        self.next_token(); // fir
        // println!("{:?}", self.current_token());
        
        let body = self.parse_block();

        self.next_token(); // wapas karo
        // println!("{:?}", self.current_token());

        let return_data = self.parse_expression();

        Statement::FunctionDef(func_name, params, body, return_data)
    }

    fn parse_function_call(&mut self) -> Statement {

        self.next_token(); // chalao
        // println!("{:?}", self.current_token());
        
        let func_name = if let Token::Identifier(name) = self.current_token() {
            name.clone()
        } else {
            panic!("Expected identifier after 'mana'")
        };
        self.next_token(); // func_name
        // println!("{:?}", self.current_token());
        
        let mut args: Vec<Expression> = Vec::new();
        while self.current_token() != Token::FunctionReturn {
            args.push(self.parse_primary());
            self.next_token(); 
            // println!("{:?}", self.current_token());
        }

        self.next_token(); // par
        // println!("{:?}", self.current_token());
        
        let var_name = if let Token::Identifier(name) = self.current_token() {
            name.clone()
        } else {
            panic!("Expected identifier after 'mana'")
        };
        self.next_token(); // return_var
        // println!("{:?}", self.current_token());

        if self.current_token() != Token::FunctionCallEnd {
            panic!("Invalid Syntax")
        } else {
            self.next_token(); //  me
        }

        Statement::FunctionCall(func_name, args, var_name)
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        let statement = match self.current_token() {
            Token::Let => Some(self.parse_declaration()),
            Token::If => Some(self.parse_if()),
            Token::While => Some(self.parse_while()),
            Token::Print => Some(self.parse_print()),
            Token::Input => Some(self.parse_input()),
            Token::FunctionDef => Some(self.parse_function_def()),
            Token::FunctionCallStart => Some(self.parse_function_call()),
            _ => None,
        };
        statement
    }

    fn parse_declaration(&mut self) -> Statement {
        self.next_token();
        // println!("{:?}", self.current_token());

        let var_name = if let Token::Identifier(name) = self.current_token() {
            name.clone()
        } else {
            panic!("Expected identifier after 'mana'")
        };
        self.next_token();
        // println!("{:?}", self.current_token());

        if self.current_token() != Token::Assign {
            panic!("Invalid syntax")
        }

        self.next_token();
        // println!("{:?}", self.current_token());

        let expr = self.parse_expression();

        Statement::Declaration(var_name, expr)
    }

    fn parse_if(&mut self) -> Statement {
        self.next_token();
        // println!("{:?}", self.current_token());

        let condition = self.parse_expression();

        let then_block = self.parse_block();

        let else_block = if let Token::Else = self.current_token() {
            self.next_token();
            // println!("{:?}", self.current_token());

            self.parse_block()
        } else {
            Vec::new()
        };
        Statement::If {
            condition,
            then_block,
            else_block,
        }
    }

    fn parse_while(&mut self) -> Statement {
        self.next_token();
        // println!("{:?}", self.current_token());

        let condition = self.parse_expression();
        let body = self.parse_block();
        Statement::While { condition, body }
    }

    fn parse_print(&mut self) -> Statement {
        // println!("{:?}", self.current_token());
        self.next_token();
        // println!("{:?}", self.current_token());

        let expr = self.parse_expression();

        Statement::Print(expr)
    }

    fn parse_input(&mut self) -> Statement {
        self.next_token();
        // println!("{:?}", self.current_token());

        let var_name = if let Token::Identifier(name) = self.current_token() {
            name.clone()
        } else {
            panic!("Expected identifier after 'padho'")
        };
        self.next_token();
        // println!("{:?}", self.current_token());
        let var_type = if let Token::Identifier(name) = self.current_token() {
            name.clone()
        } else {
            panic!("Expected identifier after 'padho'")
        };

        self.next_token();
        // println!("{:?}", self.current_token());

        Statement::Input(var_name, var_type)
    }

    fn parse_expression(&mut self) -> Expression {
        // This is a simplified version, only handles binary operations and literals for now

        let left = self.parse_primary();
        self.next_token();
        // println!("{:?}", self.current_token());

        let right = self.parse_primary();
        self.next_token();
        // println!("{:?}", self.current_token());
        
        match right {
            Expression::None => {
                if self.current_token() != Token::TheEnd {
                    // println!("{:?}", self.current_token());
                    self.previous_token();
                }
                left
            }
            _ => {
                if self.has_next() {
                    let op = self.parse_binary_operator().unwrap();
                    self.next_token();
                    // println!("{:?}", self.current_token());
                    Expression::BinaryOp(Box::new(left), op, Box::new(right))
                } else {
                    left
                }
            }
        }
    }

    fn parse_primary(&mut self) -> Expression {
        match self.current_token() {
            Token::Integer(value) => Expression::Integer(value),
            Token::Float(value) => Expression::Float(value),
            Token::String(value) => Expression::String(value),
            Token::Identifier(name) => Expression::Identifier(name),
            Token::True => Expression::Boolean(true),
            Token::False => Expression::Boolean(false),
            _ => Expression::None,
        }
    }

    fn parse_binary_operator(&mut self) -> Option<BinOp> {
        match self.current_token() {
            Token::Plus => Some(BinOp::Plus),
            Token::Minus => Some(BinOp::Minus),
            Token::Multiply => Some(BinOp::Multiply),
            Token::Divide => Some(BinOp::Divide),
            Token::Modulo => Some(BinOp::Modulo),
            Token::LessThan => Some(BinOp::LessThan),
            Token::GreaterThan => Some(BinOp::GreaterThan),
            Token::LessThanOrEqual => Some(BinOp::LessThanOrEqual),
            Token::GreaterThanOrEqual => Some(BinOp::GreaterThanOrEqual),
            Token::Equals => Some(BinOp::Equals),
            Token::NotEquals => Some(BinOp::NotEquals),
            _ => None,
        }
    }

    fn parse_block(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while self.current_token() != Token::BlockEnd {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            }

            if self.current_token() == Token::Else {
                break;
            }

            if self.current_token() == Token::BlockEnd {
                self.next_token();
                break;
            }

            if self.current_token() == Token::Return {
                break;
            }
        }
        statements
    }
}
