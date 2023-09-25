use cfront_definition_keyword::Keyword;

#[derive(Debug, PartialEq, Eq, Clone, )]
pub enum TokenType <'a> {

    Parenthesis { is_left: bool }, 
    Brace { is_left: bool }, 
    Bracket { is_left: bool }, 

    Operator (&'a str ), 

    Identifier (&'a str ), 
    NumberLiteral(&'a str, Option<&'a str>),

    StringLiteral (&'a str, bool ), 
    CharLiteral (&'a str, bool ), 

    Keyword (Keyword ), 

}


#[derive(Debug, PartialEq, Eq, Clone, )] 
pub struct Token <'a> {
    pub token_type: TokenType<'a>, 
    pub line: usize, 
    pub column: usize, 
}