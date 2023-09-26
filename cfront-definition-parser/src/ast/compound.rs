use cfront_definition::{token::{Token, TokenType}, Keyword};

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

impl BinaryExpressionType {
    pub fn upper(self) -> Option<Self> {
        match self {
            BinaryExpressionType::Multiplicative => None, 
            BinaryExpressionType::Additive => Some(BinaryExpressionType::Multiplicative), 
            BinaryExpressionType::Shift => Some(BinaryExpressionType::Additive), 
            BinaryExpressionType::Relational => Some(BinaryExpressionType::Shift), 
            BinaryExpressionType::Equality => Some(BinaryExpressionType::Relational), 
            BinaryExpressionType::And => Some(BinaryExpressionType::Equality), 
            BinaryExpressionType::ExclusiveOr => Some(BinaryExpressionType::And), 
            BinaryExpressionType::InclusiveOr => Some(BinaryExpressionType::ExclusiveOr), 
            BinaryExpressionType::LogicalAnd => Some(BinaryExpressionType::InclusiveOr), 
            BinaryExpressionType::LogicalOr => Some(BinaryExpressionType::LogicalAnd), 
        } 
    }
}

#[derive(Debug, PartialEq, Eq, Clone)] 
pub enum UnaryExpressionType {
    Plus, 
    Minus, 
    BitwiseNot, 
    LogicalNot, 
    Dereference, 
    Address,  
    Increment, 
    Decrement, 
    SizeofType, 
    SizeofExpression,
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
    PostfixExp(PostfixExpression<'a>), 
    UnaryExp(UnaryExpressionType, Box<Expression<'a>>), 
}

#[allow(unreachable_code)]
#[derive(Debug, PartialEq, Eq, Clone)] 
pub enum PostfixExpression <'a> {
    ArgumentCall(!), 
    VoidCall(Box<Expression<'a>>),
    Index(Box<Expression<'a>>, Box<Expression<'a>>), 
    Member(Box<Expression<'a>>, Token<'a>), 
    PointerMember(Box<Expression<'a>>, Token<'a>), 
    Increment(Box<Expression<'a>>), 
    Decrement(Box<Expression<'a>>),  
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
    return parse_expression_level(input_tokens, BinaryExpressionType::LogicalOr); 
} 

pub fn parse_conditional_expression <'a> (input_tokens: &'a [Token<'a>]) -> Result<(Expression<'a>, &'a [Token<'a>]), ()> {
    
    todo!()
}

pub fn parse_expression_level <'a> (input_tokens: &'a [Token<'a>], label: BinaryExpressionType) -> Result<(Expression<'a>, &'a [Token<'a>]), ()> {
    let upper = label.upper(); 
    let p: Box<dyn Fn(&'a [Token<'a>]) -> Result<(Expression<'a>, &'a [Token<'a>]), ()>> = match upper {
        Some(upper) => {
            let k = move |i| {
                parse_expression_level(i, upper) 
            };
            Box::new(k) 
        }
        None => {
            Box::new(parse_cast_expression)
        }
    };
    let (nxt, mut rst) = p(input_tokens)?;
    let mut current = vec![(nxt, TokenType::Operator(""))]; 
    loop {
        let nxt_op = rst.first(); 
        match nxt_op {
            Some(op) => {
                let t = operator_type(&op.token_type); 
                match t {
                    Some(level) => {
                        if level < label {
                            break ;                    
                        } else{
                            let Ok((second, r)) = p(&rst[1..]) else { break }; 
                            rst = r; 
                            current.push((second, op.token_type.clone())); 
                        }
                    },
                    None => break, 
                }
            }
            None => break, 
        }
    }
    if current.len() == 1 {
        return Ok((current.pop().unwrap().0, rst)); 
    } else {
        return Ok((Expression {
            expression_type: ExpressionType::BinaryExps(current, label), 
            token_slice: &input_tokens[..=input_tokens.len() - rst.len()], 
        }, rst)); 
    }
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
    let mut idx; 
    'post_parse: loop {
        let first = input.first(); 
        let Some(first) = first else {
            return Ok((r, input)); 
        }; 
        idx = input_tokens.len() - input.len(); 
        let ttype = &first.token_type; 
        match ttype {
            TokenType::Parenthesis { is_left: true } => {
                // empty argument check 
                let second = input.get(1).ok_or(())?; 
                let stype = &second.token_type; 
                match stype {
                    TokenType::Parenthesis { is_left: false } => {
                        r = Expression {
                            expression_type: ExpressionType::PostfixExp(PostfixExpression::VoidCall(Box::new(r))),
                            token_slice: &input_tokens[..idx], 
                        }; 
                        input = &input[2..]; 
                    }
                    _ => {
                        unimplemented!("Not support argument call yet ")
                    }
                } 
            }
            TokenType::Bracket { is_left: true } => {
                let (exp, rest) = parse_expression(&input[1..])?; 
                let second = rest.first().ok_or(())?; 
                let stype = &second.token_type; 
                match stype {
                    TokenType::Bracket { is_left: false } => {
                        r = Expression {
                            expression_type: ExpressionType::PostfixExp(PostfixExpression::Index(Box::new(r), Box::new(exp))),
                            token_slice: &input_tokens[..idx], 
                        }; 
                        input = &rest[1..]; 
                    }
                    _ => return Err(()), 
                }
            }
            TokenType::Operator("++") | TokenType::Operator("--") => {
                input = &input[1..]; 
                idx += 1; 
                r = Expression {
                    expression_type: ExpressionType::PostfixExp(match ttype {
                        TokenType::Operator("++") => PostfixExpression::Increment(Box::new(r)), 
                        TokenType::Operator("--") => PostfixExpression::Decrement(Box::new(r)), 
                        _ => unreachable!(), 
                    }), 
                    token_slice: &input_tokens[..idx], 
                }; 
            } 
            TokenType::Operator(".") | TokenType::Operator("->") => {
                let is_ptr = ttype == &TokenType::Operator("->"); 
                let second = input.get(1); 
                let Some(second) = second else { 
                    break 'post_parse; 
                }; 
                let stype = &second.token_type; 
                match stype {
                    TokenType::Identifier(_) => {
                        idx += 2; 
                        input = &input[2..]; 
                        r = Expression {
                            expression_type: ExpressionType::PostfixExp(match is_ptr {
                                true => PostfixExpression::PointerMember(Box::new(r), second.clone()), 
                                false => PostfixExpression::Member(Box::new(r), second.clone()), 
                            }), 
                            token_slice: &input_tokens[..idx], 
                        };  
                    }
                    _ => break 'post_parse, 
                }
            } 
            _ => break 'post_parse, 
        } 
    } 
    return Ok((r, input)); 
} 

pub fn parse_unary_expression<'a> (input_tokens: &'a [Token<'a>]) -> Result<(Expression<'a>, &'a [Token<'a>]), ()> {
    let first = input_tokens.first().ok_or(())?; 
    let ft = &first.token_type;
    let ans = match ft {
        TokenType::Operator("++") | TokenType::Operator("--") => {
            let plus = ft == &TokenType::Operator("++"); 
            let (exp, rest) = parse_unary_expression(&input_tokens[1..])?; 
            let idx = input_tokens.len() - rest.len(); 
            let expression_type = match plus {
                true => ExpressionType::UnaryExp(UnaryExpressionType::Increment, Box::new(exp)), 
                false => ExpressionType::UnaryExp(UnaryExpressionType::Decrement, Box::new(exp)), 
            }; 
            (Expression {
                expression_type, 
                token_slice: &input_tokens[..=idx], 
            }, rest)
        }
        TokenType::Keyword(Keyword::Sizeof) => {
            let second = input_tokens.get(1).ok_or(())?; 
            let st = &second.token_type; 
            _ = st;
            let (r, i) = parse_unary_expression(&input_tokens[1..])?; 
            let idx = input_tokens.len() - i.len(); 
            let expression_type = ExpressionType::UnaryExp(UnaryExpressionType::SizeofExpression, Box::new(r)); 
            (Expression {
                expression_type, 
                token_slice: &input_tokens[..=idx], 
            }, i) 
        }
        | TokenType::Operator("+") 
        | TokenType::Operator("-") 
        | TokenType::Operator("~") 
        | TokenType::Operator("!") 
        | TokenType::Operator("*") 
        | TokenType::Operator("&") 
        => {
            let (e, input) = parse_cast_expression(&input_tokens[1..])?;  
            let idx = input_tokens.len() - input.len(); 
            let t = match ft {
                TokenType::Operator("+") => UnaryExpressionType::Plus, 
                TokenType::Operator("-") => UnaryExpressionType::Minus, 
                TokenType::Operator("~") => UnaryExpressionType::BitwiseNot, 
                TokenType::Operator("!") => UnaryExpressionType::LogicalNot, 
                TokenType::Operator("*") => UnaryExpressionType::Dereference, 
                TokenType::Operator("&") => UnaryExpressionType::Address, 
                _ => unreachable!(), 
            }; 
            let expression_type = ExpressionType::UnaryExp(t, Box::new(e)); 
            (Expression {
                expression_type,
                token_slice: &input_tokens[..=idx],
            }, input)
        }
        _ => parse_post_expression(input_tokens)?, 
    }; 
    return Ok(ans);
}

pub fn parse_cast_expression <'a> (input_tokens: &'a [Token<'a>]) -> Result<(Expression<'a>, &'a [Token<'a>]), ()> {
    return parse_unary_expression(input_tokens);
}
