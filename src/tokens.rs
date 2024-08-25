use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    #[token("mana")]
    Let,
    #[token("agar")]
    If,
    #[token("warna")]
    Else,
    #[token("jabtak")]
    While,
    #[token("aage")]
    BlockEnd,
    #[token(".")]
    StatementEnd,
    #[token("samapt")]
    TheEnd,

    // Literals
    #[regex(r"[0-9]+", |lex| lex.slice().parse().ok())]
    Integer(i64),
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse().ok())]
    Float(f64),
    #[token("satya")]
    True,
    #[token("asatya")]
    False,
    #[regex(r#""[^"]*""#, |lex| Some(lex.slice().trim_matches('"').to_string()))]
    String(String),

    // Identifiers
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| Some(lex.slice().to_string()))]
    Identifier(String),

    // Operators
    #[token("ka yog")]
    Plus,
    #[token("ka antar")]
    Minus,
    #[token("ka guna")]
    Multiply,
    #[token("ka bhaag")]
    Divide,
    #[token("%")]
    Modulo,
    #[token("barabar")]
    Assign,
    #[token("hai")]
    Equals,
    #[token("nhi hai")]
    NotEquals,
    #[token("se chota hai")]
    LessThan,
    #[token("se bada hai")]
    GreaterThan,
    #[token("se chota hai ya barabar hai")]
    LessThanOrEqual,
    #[token("se bada hai ya barabar hai")]
    GreaterThanOrEqual,
    #[token("aur")]
    And,
    #[token("ya")]
    Or,
    #[token("nhi")]
    Not,

    // Delimiters
    #[token(",")]
    Comma,
    #[token("\"")]
    DoubleQuote,
    #[token("'")]
    SingleQuote,
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,

    // Ignore whitespace
    #[regex(r"\s+", logos::skip)]
    Whitespace,

    // Ignore comments
    #[regex(r"faltu[^\n]*", logos::skip)]
    Comment,

    #[token("likho")]
    Print,
    #[token("padho")]
    Input,
}

impl Token {
    pub fn tokenize<'a>(source: &'a str) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let lexer = Token::lexer(source);
        for token in lexer {
            tokens.push(token.unwrap());
        }
        tokens
    }    
}