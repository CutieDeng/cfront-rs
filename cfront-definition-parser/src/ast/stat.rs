use cfront_definition::{token::{Token, TokenType, self}, Keyword};

use crate::{Parser, ast::{const_exp::ConstExp, AstType, exp::Exp, decl::DeclList}};

use super::Ast;

#[derive(Debug, PartialEq, Eq, Clone)] 
pub enum StatTag { 
    LabeledStat,
    ExpStat,
    CompoundStat,
    SelectionStat,
    IterationStat,
    JumpStat,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Stat<'a> (pub StatTag, pub Box<Ast<'a>>);

impl <'a> Parser<'a> for Stat<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [cfront_definition::token::Token<'a>]) -> Result<(Self, &'a [cfront_definition::token::Token<'a>]), <Self as Parser<'a>>::E> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LabeledStat<'a> {
    Label {
        id: Token<'a>, 
        stat: Box<Ast<'a>>,
    },
    Case {
        const_exp: Box<Ast<'a>>,
        stat: Box<Ast<'a>>,
    }, 
    Default {
        stat: Box<Ast<'a>>,
    }, 
}

impl <'a> Parser<'a> for LabeledStat<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let first = tokens.first().ok_or(())?;
        let ft = &first.token_type;
        let rs = &tokens[1..]; 
        match ft {
            TokenType::Identifier(_) => {
                let nxt = rs.first().ok_or(())?; 
                let nxtt = &nxt.token_type; 
                let TokenType::Operator(":") = nxtt else { return Err(()) }; 
                let rs2 = &rs[1..]; 
                let (stat, rs3) = Stat::parse(stack, rs2)?; 
                let stat = Box::new(Ast(AstType::Stat(stat), &rs2[..rs2.len() - rs3.len()])); 
                return Ok((LabeledStat::Label { id: first.clone(), stat }, rs3)); 
            }
            TokenType::Keyword(Keyword::Case) => {
                let (const_exp, rs2) = ConstExp::parse(stack, rs)?;
                let nxt = rs2.first().ok_or(())?; 
                let nxtt = &nxt.token_type; 
                let TokenType::Operator(":") = nxtt else { return Err(()) }; 
                let rs3 = &rs2[1..]; 
                let (stat, rs4) = Stat::parse(stack, rs3)?; 
                let const_exp = Box::new(Ast(AstType::ConstExp(const_exp), &rs[..rs.len() - rs2.len()])); 
                let stat = Box::new(Ast(AstType::Stat(stat), &rs3[..rs3.len() - rs4.len()])); 
                return Ok((LabeledStat::Case { const_exp, stat }, rs4)); 
            }
            TokenType::Keyword(Keyword::Default) => {
                let nxt = rs.first().ok_or(())?; 
                let nxtt = &nxt.token_type; 
                let TokenType::Operator(":") = nxtt else { return Err(()) }; 
                let rs2 = &rs[1..]; 
                let (stat, rs3) = Stat::parse(stack, rs2)?; 
                let stat = Box::new(Ast(AstType::Stat(stat), &rs2[..rs2.len() - rs3.len()])); 
                return Ok((LabeledStat::Default { stat }, rs3)); 
            } 
            _ => return Err(()), 
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)] 
pub struct ExpStat<'a> (pub Option<Box<Ast<'a>>>); 

impl <'a> Parser<'a> for ExpStat<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let f = tokens.first().ok_or(())?; 
        let ft = &f.token_type; 
        if let TokenType::Operator(";") = ft {
            return Ok((ExpStat(None), &tokens[1..])); 
        } 
        let (p, r) = Exp::parse(stack, tokens)?; 
        let f = r.first().ok_or(())?;
        let ft = &f.token_type; 
        let TokenType::Operator(";") = ft else { return Err(()) }; 
        let p = Box::new(Ast(AstType::Exp(p), &tokens[..tokens.len() - r.len()])); 
        return Ok((ExpStat(Some(p)), &r[1..])); 
    }
} 

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CompoundStat <'a> {
    pub decl_list: Option<Box<Ast<'a>>>,
    pub stat_list: Option<Box<Ast<'a>>>, 
}

impl <'a> Parser<'a> for CompoundStat<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let f = tokens.first().ok_or(())?; 
        let ft = &f.token_type; 
        let TokenType::Brace { is_left: true } = ft else { return Err(()) }; 
        let mut rst = &tokens[1..]; 
        let mut decl_list = None; 
        let mut stat_list = None; 
        let d = DeclList::parse(stack, rst); 
        if let Ok((d, r2)) = d {
            let d = Box::new(Ast(AstType::DeclList(d), &rst[..rst.len() - r2.len()])); 
            rst = r2; 
            decl_list = Some(d);  
        }
        let s = StatList::parse(stack, rst); 
        if let Ok((s, r2)) = s {
            let s = Box::new(Ast(AstType::StatList(s), &rst[..rst.len() - r2.len()])); 
            rst = r2; 
            stat_list = Some(s);  
        }
        let f = rst.first().ok_or(())?; 
        let ft = &f.token_type; 
        let TokenType::Brace { is_left: false } = ft else { return Err(()) }; 
        return Ok((CompoundStat { decl_list, stat_list }, &rst[1..])); 
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StatList<'a> (pub Vec<Ast<'a>>); 

impl <'a> Parser<'a> for StatList<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let mut rst = tokens; 
        let mut ans = Vec::new(); 
        loop {
            let parse = Stat::parse(stack, rst); 
            let Ok(parse) = parse else { break }; 
            let (parse, rst2) = parse; 
            ans.push(Ast(AstType::Stat(parse), &rst[..rst.len() - rst2.len()])); 
            rst = rst2; 
        } 
        return Ok((StatList(ans), rst)); 
    }
} 

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SelectionStat <'a> {
    If {
        exp: Box<Ast<'a>>,
        stat: Box<Ast<'a>>,
        else_stat: Option<Box<Ast<'a>>>, 
    }, 
    Switch {
        exp: Box<Ast<'a>>,
        stat: Box<Ast<'a>>,
    }, 
}

impl <'a> Parser<'a> for SelectionStat<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let f = tokens.first().ok_or(())?;
        let ft = &f.token_type;
        let rs = &tokens[1..]; 
        match ft {
            TokenType::Keyword(Keyword::If) => {
                let f2 = rs.first().ok_or(())?;
                let f2t = &f2.token_type; 
                let TokenType::Parenthesis { is_left: true } = f2t else { return Err(()) }; 
                let rs = &rs[1..]; 
                let (exp, rs2) = Exp::parse(stack, rs)?; 
                let f3 = rs2.first().ok_or(())?; 
                let f3t = &f3.token_type; 
                let TokenType::Parenthesis { is_left: false } = f3t else { return Err(()) }; 
                let r3 = &rs2[1..]; 
                let (stat, r4) = Stat::parse(stack, r3)?; 
                let mut rst = r4; 
                let mut else_stat = None; 
                if let Some(Token { token_type: TokenType::Keyword(Keyword::Else), .. }) = r4.first() {
                    let r5 = &r4[1..]; 
                    let (es, r6) = Stat::parse(stack, r5)?; 
                    rst = r6; 
                    else_stat = Some(Box::new(Ast(AstType::Stat(es), &r5[..r5.len() - r6.len()]))); 
                }
                let exp = Box::new(Ast(AstType::Exp(exp), &rs[..rs.len() - rs2.len()])); 
                let stat = Box::new(Ast(AstType::Stat(stat), &r3[..r3.len() - r4.len()])); 
                return Ok((SelectionStat::If { exp, stat, else_stat }, rst)); 
            }
            TokenType::Keyword(Keyword::Switch) => {
                let f = rs.first().ok_or(())?; 
                let ft = &f.token_type; 
                let TokenType::Parenthesis { is_left: true } = ft else { return Err(()) }; 
                let rs2 = &rs[1..]; 
                let (exp, rs3) = Exp::parse(stack, rs2)?; 
                let f2 = rs3.first().ok_or(())?; 
                let ft2 = &f2.token_type; 
                let TokenType::Parenthesis { is_left: false } = ft2 else { return Err(()) }; 
                let rs4 = &rs3[1..]; 
                let (stat, rs5) = Stat::parse(stack, rs4)?; 
                let exp = Box::new(Ast(AstType::Exp(exp), &rs2[..rs2.len() - rs3.len()])); 
                let stat = Box::new(Ast(AstType::Stat(stat), &rs4[..rs4.len() - rs5.len()])); 
                return Ok((SelectionStat::Switch { exp, stat }, rs5)); 
            }
            _ => return Err(()), 
        }
        todo!()
    }
}