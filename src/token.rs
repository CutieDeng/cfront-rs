use self::keyword::Keyword;

#[derive(Debug)]
pub enum TokenType <'a> {

    Parenthesis { is_left: bool }, 
    Brace { is_left: bool }, 
    Bracket { is_left: bool }, 

    Operator (&'a str ), 

    Identifier (&'a str ), 
    IntegerLiteral (&'a str, Option<&'a str>), 
    FloatLiteral (&'a str ), 
    StringLiteral (&'a str ), 
    CharLiteral (&'a str ), 
    Keyword (Keyword ), 

}

#[derive(Debug)]
pub struct Token<'a> {
    pub token_type: TokenType<'a>, 
    pub line: usize, 
    pub column: usize, 
} 

pub mod tokenize;
pub mod keyword; 