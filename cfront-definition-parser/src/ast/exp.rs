use cfront_definition::{token::{Token, TokenType, self}, Keyword};

use crate::{Parser, ast::{AstType, type_name::TypeName, argument_exp_list::ArgumentExpList}};

use super::Ast;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Exp <'a> {
    i : &'a !, 
}

impl <'a> Parser<'a> for Exp<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AssignmentExp<'a> {
    ConditionalExp(Box<Ast<'a>>),
    Assign {
        unary_exp: Box<Ast<'a>>, 
        assignment_op: Token<'a>,
        assignment_exp: Box<Ast<'a>>, 
    }
}

impl <'a> Parser<'a> for AssignmentExp<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        
        todo!()
    }
} 

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UnaryExp<'a> {
    PostfixExp(Box<Ast<'a>>),
    PreExp(Token<'a>, Box<Ast<'a>>),
    UnaryOp(Token<'a>, Box<Ast<'a>>), 
    SizeOfUnaryExp(Box<Ast<'a>>), 
    SizeOfTypeName(Box<Ast<'a>>), 
}

impl <'a> UnaryExp<'a> {
    pub fn parse_by_type_name(stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), ()> {
        let f = tokens.first().ok_or(())?; 
        let ft = &f.token_type; 
        let TokenType::Keyword(Keyword::Sizeof) = ft else { return Err(()) }; 
        let f = tokens.get(1).ok_or(())?; 
        let ft = &f.token_type; 
        let TokenType::Parenthesis { is_left: true } = ft else { return Err(()) }; 
        let r = &tokens[2..]; 
        let (p, r2) = TypeName::parse(stack, r)?;
        let f = r2.first().ok_or(())?; 
        let ft = &f.token_type; 
        let TokenType::Parenthesis { is_left: false } = ft else { return Err(()) }; 
        let p = Ast(AstType::TypeName(p), &r[..r.len() - r2.len()]);
        Ok((Self::SizeOfTypeName(Box::new(p)), r)) 
    } 
}

impl <'a> Parser<'a> for UnaryExp<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let f = tokens.first().ok_or(())?; 
        let ft = &f.token_type; 
        match ft {
            | TokenType::Operator("++") 
            | TokenType::Operator("--") 
            => {
                let (p, r) = UnaryExp::parse(stack, &tokens[1..])?;
                let p = Ast(AstType::UnaryExp(p), &tokens[..tokens.len() - r.len()]); 
                return Ok((Self::PreExp(f.clone(), Box::new(p)), r)); 
            },
            | TokenType::Operator("&") 
            | TokenType::Operator("*") 
            | TokenType::Operator("+") 
            | TokenType::Operator("-") 
            | TokenType::Operator("~") 
            | TokenType::Operator("!") 
            => {
                let (p, r) = UnaryExp::parse(stack, &tokens[1..])?;
                let p = Ast(AstType::UnaryExp(p), &tokens[..tokens.len() - r.len()]); 
                return Ok((Self::UnaryOp(f.clone(), Box::new(p)), r)); 
            }, 
            | TokenType::Keyword(Keyword::Sizeof) 
            => {
                let rs = &tokens[1..]; 
                let ue = UnaryExp::parse(stack, rs); 
                let tn = Self::parse_by_type_name(stack, rs);
                let select_ue; 
                match (&ue, &tn) {
                    (Ok((_, l1)), Ok((_, l2))) => select_ue = l1.len() < l2.len(),
                    (Ok(_), Err(_)) => select_ue = true, 
                    (Err(_), Ok(_)) => select_ue = false, 
                    (Err(_), Err(_)) => return Err(()), 
                }
                if select_ue {
                    let u = ue.unwrap();
                    let p = Ast(AstType::UnaryExp(u.0), &rs[..rs.len() - u.1.len()]);
                    return Ok((Self::SizeOfUnaryExp(Box::new(p)), u.1)); 
                } else { 
                    let t = tn.unwrap();
                    return Ok(t); 
                }
            }
            _ => (),
        }
        let (p, r) = PostfixExp::parse(stack, tokens)?; 
        let p = Ast(AstType::PostfixExp(p), &tokens[..tokens.len() - r.len()]); 
        Ok((Self::PostfixExp(Box::new(p)), r)) 
    }
} 

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ConditionalExp<'a> {
    Condition {
        logical_or_exp: Box<Ast<'a>>, 
        exp: Box<Ast<'a>>, 
        conditional_exp: Box<Ast<'a>>, 
    }, 
    LogicalOrExp(Box<Ast<'a>>), 
}

impl <'a> Parser<'a> for ConditionalExp<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        todo!()
    }
} 

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BiExp<'a> {
    pub level: BiOperatorLevel, 
    pub exps: Vec<(Ast<'a>, Token<'a>)>,
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Copy)]
pub enum BiOperatorLevel {
    LogicalOrExp, 
    LogicalAndExp, 
    InclusiveOrExp, 
    ExclusiveOrExp, 
    AndExp, 
    EqualityExp, 
    RelationalExp, 
    ShiftExp, 
    AdditiveExp, 
    MultiplicativeExp,  
}

impl BiOperatorLevel {
    pub fn level_up(self) -> Option<Self> {
        match self {
            Self::LogicalOrExp => Some(Self::LogicalAndExp), 
            Self::LogicalAndExp => Some(Self::InclusiveOrExp), 
            Self::InclusiveOrExp => Some(Self::ExclusiveOrExp), 
            Self::ExclusiveOrExp => Some(Self::AndExp), 
            Self::AndExp => Some(Self::EqualityExp), 
            Self::EqualityExp => Some(Self::RelationalExp), 
            Self::RelationalExp => Some(Self::ShiftExp), 
            Self::ShiftExp => Some(Self::AdditiveExp), 
            Self::AdditiveExp => Some(Self::MultiplicativeExp), 
            Self::MultiplicativeExp => None, 
        } 
    }
    pub fn level_down(self) -> Option<Self> {
        match self {
            Self::LogicalOrExp => None, 
            Self::LogicalAndExp => Some(Self::LogicalOrExp), 
            Self::InclusiveOrExp => Some(Self::LogicalAndExp), 
            Self::ExclusiveOrExp => Some(Self::InclusiveOrExp), 
            Self::AndExp => Some(Self::ExclusiveOrExp), 
            Self::EqualityExp => Some(Self::AndExp), 
            Self::RelationalExp => Some(Self::EqualityExp), 
            Self::ShiftExp => Some(Self::RelationalExp), 
            Self::AdditiveExp => Some(Self::ShiftExp), 
            Self::MultiplicativeExp => Some(Self::AdditiveExp), 
        }  
    }
    pub fn match_operator(self, op: &TokenType) -> bool {
        match op {
            TokenType::Operator(op) => match self {
                Self::LogicalOrExp => *op == "||", 
                Self::LogicalAndExp => *op == "&&", 
                Self::InclusiveOrExp => *op == "|", 
                Self::ExclusiveOrExp => *op == "^", 
                Self::AndExp => *op == "&", 
                Self::EqualityExp => *op == "==" || *op == "!=", 
                Self::RelationalExp => *op == "<" || *op == ">" || *op == "<=" || *op == ">=", 
                Self::ShiftExp => *op == "<<" || *op == ">>", 
                Self::AdditiveExp => *op == "+" || *op == "-", 
                Self::MultiplicativeExp => *op == "*" || *op == "/" || *op == "%",
            }, 
            _ => false, 
        } 
    }
}

impl <'a> Parser<'a> for BiExp<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        todo!()
    }
}

impl <'a> BiExp<'a> {
    pub fn parse_impl(tokens: &'a [Token<'a>], level: BiOperatorLevel) -> Result<(Self, &'a [Token<'a>]), ()> {
        let mut ans = Vec::new(); 
        let mut op;
        let mut rst = tokens; 
        let up = level.level_up();
        op = Token { token_type: TokenType::Operator(""), line: 0, column: 0 };
        loop {
            if let Some(up) = up {
                let (p, r2) = Self::parse_impl(rst, up)?;
                let p = Ast(AstType::BiExp(p), &rst[..rst.len() - r2.len()]);
                ans.push((p, op));
                rst = r2; 
            } else {
                let (p, r2) = CastExp::parse(&mut Vec::new(), rst)?; 
                let p = Ast(AstType::CastExp(p), &rst[..rst.len() - r2.len()]); 
                ans.push((p, op)); 
                rst = r2; 
            }
            let Some(f) = rst.first() else { break };
            let ft = &f.token_type; 
            if level.match_operator(ft) {
                op = f.clone(); 
                rst = &rst[1..]; 
            } else {
                break; 
            }
        }
        if ans.len() == 1 {
            let pop = ans.pop().unwrap().0;
            let p = match pop.0 {
                AstType::BiExp(p) => p, 
                _ => unreachable!(), 
            }; 
            Ok((p, rst))
        } else {
            Ok((Self { level, exps: ans }, rst)) 
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CastExp<'a> {
    UnaryExp(Box<Ast<'a>>), 
    Cast {
        type_name: Box<Ast<'a>>, 
        cast_exp: Box<Ast<'a>>, 
    }
}

impl <'a> Parser<'a> for CastExp<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        // try both two paths. 
        let u = UnaryExp::parse(stack, tokens);
        let u2 = CastExp::type_cast_parse(stack, tokens); 
        let select_u; 
        match (&u, &u2) {
            (Ok((_, l1)), Ok((_, l2))) => select_u = l1.len() < l2.len(), 
            (Ok(_), Err(_)) => select_u = true, 
            (Err(_), Ok(_)) => select_u = false, 
            (Err(_), Err(_)) => return Err(()), 
        }
        if select_u {
            let u = u.unwrap(); 
            let p = Ast(AstType::UnaryExp(u.0), &tokens[..tokens.len() - u.1.len()]); 
            Ok((Self::UnaryExp(Box::new(p)), u.1))
        } else {
            let u = u2.unwrap();
            let (p, r) = u; 
            Ok((p, r)) 
        }
    } 
}

impl <'a> CastExp<'a> {
    pub fn type_cast_parse(stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), ()> {
        let f = tokens.first().ok_or(())?; 
        let ft = &f.token_type; 
        let TokenType::Parenthesis { is_left: true  } = ft else { return Err(()) }; 
        let (p, r) = TypeName::parse(stack, tokens)?; 
        let f = r.first().ok_or(())?; 
        let ft = &f.token_type; 
        let TokenType::Parenthesis { is_left: false  } = ft else { return Err(()) }; 
        let r2 = &r[1..]; 
        let (p2, r3) = CastExp::parse(stack, r2)?;
        let p = Ast(AstType::TypeName(p), &tokens[..tokens.len() - r.len()]); 
        let p2 = Ast(AstType::CastExp(p2), &r2[..r2.len() - r3.len()]); 
        Ok((Self::Cast { type_name: Box::new(p), cast_exp: Box::new(p2) }, r))  
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PostfixExp<'a> {
    PrimaryExp(Box<Ast<'a>>), 
    Postfix {
        postfix_exp: Box<Ast<'a>>, 
        postfix_op: Token<'a>, 
        identity: Option<Token<'a>>,
    }, 
    FunctionCall {
        postfix_exp: Box<Ast<'a>>, 
        args: Vec<Ast<'a>>, 
    }, 
    ArraySubscript {
        postfix_exp: Box<Ast<'a>>, 
        exp: Box<Ast<'a>>, 
    }, 
}

impl <'a> Parser<'a> for PostfixExp<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let mut this; 
        let mut rst = tokens; 
        let (p, r) = PrimaryExp::parse(stack, tokens)?; 
        let p = Ast(AstType::PrimaryExp(p), &rst[..rst.len() - r.len()]); 
        this = Self::PrimaryExp(Box::new(p));  
        loop {
            let f = rst.first().ok_or(())?; 
            let ft = &f.token_type; 
            match ft {
                TokenType::Parenthesis { is_left: true } => {
                    let a = ArgumentExpList::parse(stack, rst); 
                    let v; 
                    match a {
                        Ok((l, r)) => {
                            v = l.0; 
                            rst = r; 
                        }
                        Err(_) => {
                            v = Vec::new();
                        }
                    }
                    let f = rst.first().ok_or(())?; 
                    let ft = &f.token_type; 
                    let TokenType::Parenthesis { is_left: false } = ft else { break }; 
                    let p = Ast(AstType::PostfixExp(this), &tokens[..tokens.len() - rst.len()]); 
                    this = Self::FunctionCall { postfix_exp: Box::new(p), args: v }; 
                }
                TokenType::Bracket { is_left: true } => {
                    let rs = &rst[1..]; 
                    let Ok((p, r)) = Exp::parse(stack, rs) else { break }; 
                    let f = r.first().ok_or(())?; 
                    let ft = &f.token_type; 
                    let TokenType::Bracket { is_left: false } = ft else { break }; 
                    let p = Ast(AstType::Exp(p), &rs[..rs.len() - r.len()]); 
                    let thi = Ast(AstType::PostfixExp(this), &tokens[..tokens.len() - rst.len()]); 
                    this = Self::ArraySubscript { postfix_exp: Box::new(thi), exp: Box::new(p) }; 
                    rst = &r[1..]; 
                }
                | TokenType::Operator(".")
                | TokenType::Operator("->")
                => {
                    let Some(i) = rst.get(1) else { break }; 
                    let it = &i.token_type; 
                    match it {
                        TokenType::Identifier(_) => {
                            let p = Ast(AstType::PostfixExp(this), &tokens[..tokens.len() - rst.len()]); 
                            this = Self::Postfix { postfix_exp: Box::new(p), postfix_op: f.clone(), identity: Some(i.clone()) }; 
                            rst = &rst[2..]; 
                        }
                        _ => break, 
                    } 
                }
                | TokenType::Operator("++") 
                | TokenType::Operator("--") 
                => {
                    let p = Ast(AstType::PostfixExp(this), &tokens[..tokens.len() - rst.len()]); 
                    this = Self::Postfix { postfix_exp: Box::new(p), postfix_op: f.clone(), identity: None }; 
                    rst = &rst[1..];  
                }
                _ => break, 
            }
        }
        Ok((this, rst)) 
    }
} 

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PrimaryExp<'a> {
    Id(Token<'a>), 
    Const(Token<'a>), 
    String(Vec<Token<'a>>),
    Exp(Box<Ast<'a>>), 
}

impl <'a> Parser<'a> for PrimaryExp<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let f = tokens.first().ok_or(())?;
        let ft = &f.token_type; 
        match ft {
            TokenType::Identifier(_) => {
                return Ok((Self::Id(f.clone()), &tokens[1..])); 
            }
            TokenType::NumberLiteral(_, _) => {
                return Ok((Self::Const(f.clone()), &tokens[1..])); 
            }
            TokenType::CharLiteral(_, _) => {
                return Ok((Self::Const(f.clone()), &tokens[1..]));  
            }
            TokenType::StringLiteral(_, _) => {
                let mut ans = vec![f.clone()];
                let mut idx = 1; 
                while let Some(f) = tokens.get(idx) {
                    let ft = &f.token_type; 
                    match ft {
                        TokenType::StringLiteral(_, _) => {
                            ans.push(f.clone()); 
                            idx += 1; 
                        }
                        _ => break, 
                    }
                } 
                return Ok((Self::String(ans), &tokens[idx..]));  
            }
            TokenType::Parenthesis { is_left: true } => {
                let rs = &tokens[1..]; 
                let (p, r) = Exp::parse(stack, rs)?; 
                let f = r.first().ok_or(())?; 
                let ft = &f.token_type; 
                let TokenType::Parenthesis { is_left: false } = ft else { return Err(()) }; 
                let p = Ast(AstType::Exp(p), &rs[..rs.len() - r.len()]); 
                return Ok((Self::Exp(Box::new(p)), &r[1..])); 
            }
            _ => return Err(()),
        }
    }
}  