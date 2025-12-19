use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(skip r"#[^\n]*")]
pub enum Token {
    // Keywords
    #[token("@scene")]
    Scene,
    
    #[token("@character")]
    Character,
    
    #[token("@show")]
    Show,
    
    #[token("@hide")]
    Hide,
    
    #[token("@background")]
    Background,
    
    #[token("@music")]
    Music,
    
    #[token("@sound")]
    Sound,
    
    #[token("@if")]
    If,
    
    #[token("@elif")]
    Elif,
    
    #[token("@else")]
    Else,
    
    #[token("@endif")]
    EndIf,
    
    #[token("@choice")]
    Choice,
    
    #[token("@option")]
    Option,
    
    #[token("@label")]
    Label,
    
    #[token("@jump")]
    Jump,
    
    #[token("@call")]
    Call,
    
    #[token("@var")]
    Var,
    
    #[token("@set")]
    Set,
    
    #[token("@effect")]
    Effect,
    
    #[token("@wait")]
    Wait,
    
    #[token("@video")]
    Video,
    
    #[token("@exit")]
    Exit,
    
    // Position keywords
    #[token("at")]
    At,
    
    #[token("with")]
    With,
    
    #[token("to")]
    To,
    
    #[token("from")]
    From,
    
    #[token("left")]
    Left,
    
    #[token("center")]
    Center,
    
    #[token("right")]
    Right,
    
    // String literal
    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = lex.slice();
        s[1..s.len()-1].to_string()
    })]
    String(String),
    
    // Numbers
    #[regex(r"[0-9]+", |lex| lex.slice().parse().ok())]
    Integer(i64),
    
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse().ok())]
    Float(f64),
    
    // Boolean
    #[token("true", |_| true)]
    #[token("false", |_| false)]
    Boolean(bool),
    
    // Identifier
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),
    
    // Operators
    #[token("=")]
    Assign,
    
    #[token("+")]
    Plus,
    
    #[token("-")]
    Minus,
    
    #[token("*")]
    Multiply,
    
    #[token("/")]
    Divide,
    
    #[token("==")]
    Equal,
    
    #[token("!=")]
    NotEqual,
    
    #[token(">")]
    Greater,
    
    #[token("<")]
    Less,
    
    #[token(">=")]
    GreaterEqual,
    
    #[token("<=")]
    LessEqual,
    
    // Delimiters
    #[token(":")]
    Colon,
    
    #[token(",")]
    Comma,
    
    #[token("(")]
    LeftParen,
    
    #[token(")")]
    RightParen,
    
    #[token("->")]
    Arrow,
}

pub struct Lexer<'a> {
    inner: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: Token::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().and_then(|r| r.ok())
    }
}
