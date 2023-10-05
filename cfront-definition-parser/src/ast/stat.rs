use cfront_definition::{token::{Token, TokenType}, Keyword};

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

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let f = tokens.first().ok_or(())?; 
        let ft = &f.token_type; 
        match ft {
            TokenType::Brace { is_left: true } => {
                let (compound_stat, r) = CompoundStat::parse(stack, tokens)?; 
                let compound_stat = Box::new(Ast(AstType::CompoundStat(compound_stat), &tokens[..tokens.len() - r.len()])); 
                return Ok((Stat(StatTag::CompoundStat, compound_stat), r));  
            }
            | TokenType::Keyword(Keyword::Case) 
            | TokenType::Keyword(Keyword::Default) 
            => {
                let (labeled_stat, r) = LabeledStat::parse(stack, tokens)?; 
                let labeled_stat = Box::new(Ast(AstType::LabeledStat(labeled_stat), &tokens[..tokens.len() - r.len()])); 
                return Ok((Stat(StatTag::LabeledStat, labeled_stat), r));  
            }
            | TokenType::Keyword(Keyword::Do)
            | TokenType::Keyword(Keyword::For)
            | TokenType::Keyword(Keyword::While) 
            => {
                let (iteration_stat, r) = IterationStat::parse(stack, tokens)?; 
                let iteration_stat = Box::new(Ast(AstType::IterationStat(iteration_stat), &tokens[..tokens.len() - r.len()])); 
                return Ok((Stat(StatTag::IterationStat, iteration_stat), r));    
            }
            | TokenType::Keyword(Keyword::Switch)
            | TokenType::Keyword(Keyword::If) 
            => {
                let (selection_stat, r) = SelectionStat::parse(stack, tokens)?; 
                let selection_stat = Box::new(Ast(AstType::SelectionStat(selection_stat), &tokens[..tokens.len() - r.len()])); 
                return Ok((Stat(StatTag::SelectionStat, selection_stat), r));   
            }
            | TokenType::Identifier(_) => {
                let nxt = tokens.get(1).ok_or(())?; 
                let nxtt = &nxt.token_type; 
                if let TokenType::Operator(":") = nxtt {
                    let (labeled_stat, r) = LabeledStat::parse(stack, tokens)?; 
                    let labeled_stat = Box::new(Ast(AstType::LabeledStat(labeled_stat), &tokens[..tokens.len() - r.len()])); 
                    return Ok((Stat(StatTag::LabeledStat, labeled_stat), r)); 
                }
            }
            | TokenType::Keyword(Keyword::Goto)
            | TokenType::Keyword(Keyword::Continue) 
            | TokenType::Keyword(Keyword::Break) 
            | TokenType::Keyword(Keyword::Return) 
            => {
                let (jump_stat, r) = JumpStat::parse(stack, tokens)?; 
                let jump_stat = Box::new(Ast(AstType::JumpStat(jump_stat), &tokens[..tokens.len() - r.len()])); 
                return Ok((Stat(StatTag::JumpStat, jump_stat), r)); 
            } 
            _ => (),
        }
        let (exp, r) = ExpStat::parse(stack, tokens)?; 
        let exp = Box::new(Ast(AstType::ExpStat(exp), &tokens[..tokens.len() - r.len()])); 
        return Ok((Stat(StatTag::ExpStat, exp), r)); 
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
        // #[cfg(debug_assertions)]
        // dbg!(tokens);
        let f = tokens.first().ok_or(())?; 
        let ft = &f.token_type; 
        let TokenType::Brace { is_left: true } = ft else { return Err(()) }; 
        let mut rst = &tokens[1..]; 
        let mut decl_list = None; 
        let mut stat_list = None; 
        let d = DeclList::parse(stack, rst); 
        #[cfg(debug_assertions)]
        { _ = d.as_ref().map(|(r, _)| { dbg!(r); }) }
        if let Ok((d, r2)) = d {
            let d = Box::new(Ast(AstType::DeclList(d), &rst[..rst.len() - r2.len()])); 
            rst = r2; 
            decl_list = Some(d);  
        }
        let s = StatList::parse(stack, rst); 
        #[cfg(debug_assertions)]
        dbg!(s.as_ref().map(|(a, _)| a).ok());
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
        if ans.is_empty() {
            return Err(()); 
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
    }
}

#[derive(Debug, PartialEq, Eq, Clone)] 
pub enum IterationStat <'a> {
    While {
        exp: Box<Ast<'a>>,
        stat: Box<Ast<'a>>, 
    },
    DoWhile {
        stat: Box<Ast<'a>>,
        exp: Box<Ast<'a>>, 
    }, 
    For {
        exp1: Option<Box<Ast<'a>>>,
        exp2: Option<Box<Ast<'a>>>,
        exp3: Option<Box<Ast<'a>>>,
        stat: Box<Ast<'a>>, 
    }, 
}

impl <'a> Parser<'a> for IterationStat<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let f = tokens.first().ok_or(())?; 
        let ft = &f.token_type; 
        let rs = &tokens[1..]; 
        match ft {
            TokenType::Keyword(Keyword::While) => {
                let pl = rs.first().ok_or(())?; 
                let plt = &pl.token_type; 
                let TokenType::Parenthesis { is_left: true } = plt else { return Err(()) }; 
                let (exp, r2) = Exp::parse(stack, rs)?; 
                let nxt = r2.first().ok_or(())?; 
                let nxtt = &nxt.token_type; 
                let TokenType::Parenthesis { is_left: false } = nxtt else { return Err(()) }; 
                let r3 = &r2[1..]; 
                let (stat, r4) = Stat::parse(stack, r3)?; 
                let exp = Box::new(Ast(AstType::Exp(exp), &rs[..rs.len() - r2.len()])); 
                let stat = Box::new(Ast(AstType::Stat(stat), &r3[..r3.len() - r4.len()])); 
                return Ok((IterationStat::While { exp, stat }, r4)); 
            }
            TokenType::Keyword(Keyword::Do) => {
                let (stat, r2) = Stat::parse(stack, rs)?; 
                let nxt = r2.first().ok_or(())?; 
                let nxtt = &nxt.token_type; 
                let TokenType::Keyword(Keyword::While) = nxtt else { return Err(()) }; 
                let nxt2 = r2.get(1).ok_or(())?; 
                let nxt2t = &nxt2.token_type; 
                let TokenType::Parenthesis { is_left: true } = nxt2t else { return Err(()) }; 
                let r3 = &r2[2..]; 
                let (exp, r4) = Exp::parse(stack, r3)?; 
                let nxt3 = r4.first().ok_or(())?; 
                let nxt3t = &nxt3.token_type; 
                let TokenType::Parenthesis { is_left: false } = nxt3t else { return Err(()) }; 
                let r5 = &r4[1..]; 
                let stat = Box::new(Ast(AstType::Stat(stat), &rs[..rs.len() - r2.len()])); 
                let exp = Box::new(Ast(AstType::Exp(exp), &r3[..r3.len() - r4.len()])); 
                return Ok((IterationStat::DoWhile { stat, exp }, r5)); 
            }
            TokenType::Keyword(Keyword::For) => {
                let pl = rs.first().ok_or(())?; 
                let plt = &pl.token_type; 
                let TokenType::Parenthesis { is_left: true } = plt else { return Err(()) }; 
                let mut rst = rs; 
                let mut exps = [None, None, None];
                for (i, ele) in exps.iter_mut().enumerate() {
                    if i != 0 {
                        let nxt = rst.first().ok_or(())?; 
                        let nxtt = &nxt.token_type; 
                        let TokenType::Operator(";") = nxtt else { return Err(()) }; 
                        rst = &rst[1..]; 
                    } 
                    let exp = Exp::parse(stack, rst); 
                    if let Ok((exp, r2)) = exp {
                        let exp = Box::new(Ast(AstType::Exp(exp), &rst[..rst.len() - r2.len()])); 
                        *ele = Some(exp); 
                        rst = r2; 
                    } 
                } 
                let nxt = rst.first().ok_or(())?; 
                let nxtt = &nxt.token_type; 
                let TokenType::Parenthesis { is_left: false } = nxtt else { return Err(()) }; 
                let r2 = &rst[1..]; 
                let (stat, r3) = Stat::parse(stack, r2)?; 
                let stat = Box::new(Ast(AstType::Stat(stat), &r2[..r2.len() - r3.len()])); 
                let [exp1, exp2, exp3] = exps; 
                return Ok((IterationStat::For { exp1, exp2, exp3, stat }, r3)); 
            }
            _ => return Err(()), 
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum JumpStat <'a> {
    Goto(Token<'a>),
    Continue, 
    Break, 
    Return(Option<Box<Ast<'a>>>), 
}

impl <'a> Parser<'a> for JumpStat<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let f = tokens.first().ok_or(())?;
        let ft = &f.token_type; 
        match ft {
            TokenType::Keyword(Keyword::Goto) => {
                let nxt = tokens.get(1).ok_or(())?; 
                let nxtt = &nxt.token_type; 
                let TokenType::Identifier(_) = nxtt else { return Err(()) }; 
                let nxt2 = tokens.get(2).ok_or(())?; 
                let nxt2t = &nxt2.token_type; 
                let TokenType::Operator(";") = nxt2t else { return Err(()) }; 
                return Ok((JumpStat::Goto(nxt.clone()), &tokens[3..])); 
            }
            TokenType::Keyword(Keyword::Continue) => {
                let nxt = tokens.get(1).ok_or(())?; 
                let nxtt = &nxt.token_type; 
                let TokenType::Operator(";") = nxtt else { return Err(()) }; 
                return Ok((JumpStat::Continue, &tokens[2..])); 
            }
            TokenType::Keyword(Keyword::Break) => {
                let nxt = tokens.get(1).ok_or(())?; 
                let nxtt = &nxt.token_type; 
                let TokenType::Operator(";") = nxtt else { return Err(()) }; 
                return Ok((JumpStat::Break, &tokens[2..])); 
            }
            TokenType::Keyword(Keyword::Return) => {
                let nxt = tokens.get(1).ok_or(())?; 
                let nxtt = &nxt.token_type; 
                let TokenType::Operator(";") = nxtt else { 
                    let rst = &tokens[1..]; 
                    let (exp, r) = Exp::parse(stack, rst)?; 
                    let nxt = r.first().ok_or(())?; 
                    let nxtt = &nxt.token_type; 
                    let TokenType::Operator(";") = nxtt else { return Err(()) }; 
                    let exp = Box::new(Ast(AstType::Exp(exp), &rst[..rst.len() - r.len()]));
                    return Ok((JumpStat::Return(Some(exp)), &r[1..])); 
                }; 
                return Ok((JumpStat::Return(None), &tokens[2..])); 
            }
            _ => return Err(()), 
        }
    }
} 