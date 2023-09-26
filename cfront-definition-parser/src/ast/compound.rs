use cfront_definition::token::{Token, TokenType};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)] 
pub enum BinaryExpressionType {
    Multiplicative, 
    Additive, 
    Shift, 
    Relational, 
    Equality, 
    And, 
    ExclusiveOr, 
    InclusiveOr, 
    LogicalAnd,
    LogicalOr, 
}

#[allow(unreachable_code)]
#[derive(Debug, PartialEq, Eq, Clone)] 
pub enum ExpressionType <'a> {
    /// The token here is abvious the operator, the first is omitted for aligned. 
    BinaryExps(Vec<(Expression<'a>, TokenType<'a>)>, BinaryExpressionType), 
    CompoundExp(!), 
    ConditionalExp(Box<[Expression<'a>; 3]>), 
    ParimaryExp, 
    ParenthesisExp(Box<Expression<'a>>), 
}

#[allow(unreachable_code)]
#[derive(Debug, PartialEq, Eq, Clone)] 
pub enum PostfixExpression <'a> {
    ArgumentCall(!), 
    VoidCall,
    Index(Box<Expression<'a>>, Box<Expression<'a>>), 
    Member(Box<Expression<'a>>, Token<'a>), 
    PointerMember(Box<Expression<'a>>, Token<'a>), 
    Increment, 
    Decrement,  
}

#[derive(Debug, PartialEq, Eq, Clone)] 
pub struct Expression <'a> {
    pub expression_type: ExpressionType<'a> ,  
    pub token_slice: &'a [Token<'a>], 
}

pub fn operator_type(t: &TokenType) -> Option<BinaryExpressionType> {
    match t {
        TokenType::Operator(t) => {
            let rst = match *t {
                "*" | "/" | "%" => BinaryExpressionType::Multiplicative, 
                "+" | "-" => BinaryExpressionType::Additive, 
                "<<" | ">>" => BinaryExpressionType::Shift, 
                "<" | ">" | "<=" | ">=" => BinaryExpressionType::Relational, 
                "==" | "!=" => BinaryExpressionType::Equality, 
                "&" => BinaryExpressionType::And, 
                "^" => BinaryExpressionType::ExclusiveOr, 
                "|" => BinaryExpressionType::InclusiveOr, 
                "&&" => BinaryExpressionType::LogicalAnd, 
                "||" => BinaryExpressionType::LogicalOr, 
                _ => return None, 
            }; 
            return Some(rst); 
        }
        _ => return None, 
    }
}

pub fn parse_expression <'a> (input_tokens: &'a [Token<'a>]) -> Result<(Expression<'a>, &'a [Token<'a>]), ()> {
    let mut idx = 0; 
    let mut now_type = None; 
    now_type = None; 
    while let Some(i) = input_tokens.get(idx) {
        dbg!(i); 
        'scope : {
            match now_type {
                None => {

                }
                Some(BinaryExpressionType::Multiplicative) => {

                } 
                _ => return Err(()), 
            }
            break 'scope; 
        }
        idx += 1; 
    }
    unimplemented!()
} 

pub fn parse_primirary_expression <'a> (input_tokens: &'a [Token<'a>]) -> Result<(Expression<'a>, &'a [Token<'a>]), ()> {
    let first = input_tokens.first().ok_or(())?; 
    let ft = &first.token_type; 
    let ans = match ft {
        TokenType::Parenthesis { is_left: true } => {
            let (exp, rest) = parse_expression(&input_tokens[1..])?; 
            let second = rest.first().ok_or(())?;  
            let st = &second.token_type; 
            let idx = input_tokens.len() - rest.len(); 
            match st {
                TokenType::Parenthesis { is_left: false } => {
                    (Expression {
                        expression_type: ExpressionType::ParenthesisExp(Box::new(exp)), 
                        token_slice: &input_tokens[..=idx], 
                    }, rest) 
                }
                _ => return Err(()), 
            } 
        }
        | TokenType::Identifier(_)
        | TokenType::NumberLiteral(_, _) 
        | TokenType::StringLiteral(_, _) 
        | TokenType::CharLiteral(_, _) => {
            let mut idx = 1; 
            while let Some(i) = input_tokens.get(idx) {
                match &i.token_type {
                    TokenType::Identifier(_)
                    | TokenType::NumberLiteral(_, _) 
                    | TokenType::StringLiteral(_, _) 
                    | TokenType::CharLiteral(_, _) => {
                        idx += 1; 
                    }
                    _ => break, 
                }
            } 
            (Expression {
                expression_type: ExpressionType::ParimaryExp, 
                token_slice: &input_tokens[..idx], 
            }, &input_tokens[idx..])
        }
        _ => return Err(()), 
    }; 
    return Ok(ans);
} 

pub fn parse_post_expression<'a> (input_tokens: &'a [Token<'a>]) -> Result<(Expression<'a>, &[Token<'a>]), ()> {
    let (mut r, mut input) = parse_primirary_expression(input_tokens)?; 
    'post_parse: loop {
        let first = input.first(); 
        let Some(first) = first else {
            return Ok((r, input)); 
        }; 
        let ttype = &first.token_type; 
        match ttype {
            TokenType::Parenthesis { is_left: true } => todo!(),
            TokenType::Bracket { is_left: true } => {

            }
            TokenType::Operator(_) => todo!(),
            _ => {} 
        }
        break 'post_parse; 
    }
    unimplemented!()
} 