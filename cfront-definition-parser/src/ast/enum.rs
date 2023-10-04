use cfront_definition::{token::{Token, TokenType}, Keyword};

use crate::{Parser, ast::const_exp::ConstExp};

use super::{Ast, AstType};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EnumSpec <'a> {
    pub id: Option<Token<'a>>,
    pub enumerator_list: Option<Box<Ast<'a>>>, 
}

impl <'a> Parser<'a> for EnumSpec<'a> {

    type E = ();

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let first = tokens.first().ok_or(())?;
        let ft = &first.token_type; 
        let TokenType::Keyword(Keyword::Enum) = ft else { return Err(()) }; 
        let mut r = &tokens[1..]; 
        let nxt = r.first().ok_or(())?; 
        let nxtt = &nxt.token_type; 
        let mut id = None; 
        if let TokenType::Identifier(_) = nxtt {
            id = Some(nxt.clone()); 
            r = &r[1..];  
        } 
        let nxt = r.first(); 
        let mut enumerator_list = None; 
        if let Some(Token{token_type: TokenType::Brace { is_left: true }, ..}) = nxt {
            'list_attempt: {
                let rst = &r[1..]; 
                let Ok((el, r2)) = EnumeratorList::parse(stack, rst) else { break 'list_attempt }; 
                let Some(Token { token_type: TokenType::Brace { is_left: false }, ..}) = r2.first() else { break 'list_attempt }; 
                enumerator_list = Some(Box::new(Ast(AstType::EnumeratorList(el), &rst[..rst.len() - r2.len()]))); 
                r = r2; 
            }
        } 
        if id.is_none() && enumerator_list.is_none() {
            return Err(()); 
        } 
        return Ok((EnumSpec {
            id, 
            enumerator_list, 
        }, r)); 
    }
    
} 

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EnumeratorList<'a> { 
    pub enumerators: Vec<Ast<'a>>, 
    pub trailling_comma: bool, 
}

impl <'a> Parser<'a> for EnumeratorList<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let mut rst = tokens; 
        let mut enumerators = Vec::new(); 
        let mut trailling_comma = false; 
        loop {
            let Ok((p, r)) = Enumerator::parse(stack, rst) else { break }; 
            let p = Ast(AstType::Enumerator(p), &rst[..rst.len() - r.len()]); 
            enumerators.push(p); 
            rst = r; 
            trailling_comma = false; 
            let Some(comma) = rst.first() else { break }; 
            let comma = &comma.token_type; 
            let TokenType::Operator(",") = comma else { break }; 
            trailling_comma = true; 
            rst = &rst[1..]; 
        }
        if enumerators.is_empty() {
            return Err(()); 
        }
        let enumerator_list = EnumeratorList {
            enumerators, 
            trailling_comma, 
        }; 
        return Ok((enumerator_list, rst)); 
    }
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Enumerator<'a> {
    pub identifier: Token<'a>, 
    pub const_exp: Option<Box<Ast<'a>>>,  
}

impl <'a> Parser<'a> for Enumerator<'a> {

    type E = ();

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let first = tokens.first().ok_or(())?;
        let ft = &first.token_type; 
        let TokenType::Identifier(_) = ft else { return Err(()) };
        let mut const_exp = None; 
        let mut r = &tokens[1..]; 
        'out: {
            let Ok(second) = tokens.get(1).ok_or(()) else { break 'out };
            let st = &second.token_type; 
            let TokenType::Operator("=") = st else { break 'out }; 
            let rst = &tokens[2..]; 
            let Ok((cep, rst2)) = ConstExp::parse(stack, rst) else { break 'out }; 
            let ce = Box::new(Ast(AstType::ConstExp(cep), &rst[..rst.len() - rst2.len()])); 
            const_exp = Some(ce); 
            r = rst2; 
        }
        let enumerator = Enumerator {
            identifier: first.clone(), 
            const_exp, 
        }; 
        return Ok((enumerator, r)); 
    }

} 