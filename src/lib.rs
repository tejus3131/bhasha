mod tokens;
mod ast;
mod parser;
mod interpreter;

pub use crate::interpreter::Interpreter;
pub use crate::parser::Parser;
pub use crate::tokens::Token;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_min_if_completes() {
        let src = "mano x barabar 1 agar x 0 se bada hai likho 1 warna likho 0 aage samapt";
        let mut tokens = Token::tokenize(src);
        let mut parser = Parser::new(&mut tokens);
        let _program = parser.parse();
    }

    #[test]
    fn parse_if_else_example_completes() {
        let src = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/if_else.bhasha"))
            .expect("examples/if_else.bhasha");
        let src = format!("{src} samapt");
        let mut tokens = Token::tokenize(&src);
        let mut parser = Parser::new(&mut tokens);
        let _program = parser.parse();
    }
}