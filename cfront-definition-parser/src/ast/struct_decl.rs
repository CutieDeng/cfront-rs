use cfront_definition::token::{Token, TokenType};

use crate::{Parser, ast::{decl_specs::{TypeQualifier, TypeSpec}, AstType, declarator::Declarator, const_exp::ConstExp}};

use super::Ast;

#[derive(Debug, PartialEq, Eq, Clone)] 
pub struct StructDeclList <'a> (pub Vec<Ast<'a>>); 

impl <'a> Parser<'a> for StructDeclList<'a> {
    type E = ();

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let mut rst = tokens; 
        let mut ans = Vec::new();
        loop {
            let p = StructDecl::parse(stack, rst); 
            match p {
                Ok((p, r)) => {
                    let len = rst.len() - r.len(); 
                    ans.push(Ast(AstType::StructDecl(p), &rst[..len])); 
                    rst = r; 
                } 
                Err(_) => break, 
            } 
        }
        if ans.is_empty() {
            return Err(()); 
        } else {
            return Ok((StructDeclList(ans), rst)); 
        }
    }
}


#[derive(Debug, PartialEq, Eq, Clone)] 
pub struct StructDecl<'a> {
    pub spec_qualifier_list: Box<Ast<'a>>,
    pub struct_declarator_list: Box<Ast<'a>>, 
}

impl <'a> Parser<'a> for StructDecl<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let (sql, r1) = SpecQualifierList::parse(stack, tokens)?; 
        let (sdl, r2 ) = StructDeclaratorList::parse(stack, r1)?;
        let f = r2.first().ok_or(())?; 
        let Token { token_type: TokenType::Operator(";"), .. } = f else { return Err(()) }; 
        let len = tokens.len() - r1.len(); 
        let sql = Ast(AstType::SpecQualifierList(sql), &tokens[..len]); 
        let len = r1.len() - r2.len(); 
        let sdl = Ast(AstType::StructDeclaratorList(sdl), &r1[..len]); 
        return Ok((StructDecl { spec_qualifier_list: Box::new(sql), struct_declarator_list: Box::new(sdl) }, r2));  
    }
}

#[derive(Debug, PartialEq, Eq, Clone)] 
pub struct SpecQualifierList<'a> (pub Vec<Ast<'a>>); 

impl <'a> Parser<'a> for SpecQualifierList<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let mut rst = tokens; 
        let mut ans = Vec::new(); 
        loop {
            let tq = TypeQualifier::parse(stack, rst); 
            match tq {
                Ok((tq, r)) => {
                    let len = rst.len() - r.len(); 
                    ans.push(Ast(AstType::TypeQualifier(tq), &rst[..len])); 
                    rst = r; 
                    continue ; 
                }, 
                Err(_) => (), 
            } 
            let p = TypeSpec::parse(stack, rst); 
            match p {
                Ok((p, r)) => {
                    let len = rst.len() - r.len(); 
                    ans.push(Ast(AstType::TypeSpec(p), &rst[..len])); 
                    rst = r; 
                } 
                Err(_) => break, 
            }
        }
        if ans.is_empty() {
            return Err(());  
        } else {
            return Ok((SpecQualifierList(ans), rst));  
        }
    }
}

// struct_declarator_list
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StructDeclaratorList<'a> {
    pub struct_declarators: Vec<Ast<'a>>,
}

impl <'a> Parser<'a> for StructDeclaratorList<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let mut rst = tokens; 
        let mut ans = Vec::new(); 
        loop {
            let p = StructDeclarator::parse(stack, rst); 
            match p {
                Ok((p, r)) => {
                    let len = rst.len() - r.len(); 
                    ans.push(Ast(AstType::StructDeclarator(p), &rst[..len])); 
                    rst = r; 
                } 
                Err(_) => return Err(()), 
            }
            let comma = rst.first(); 
            if let Some(Token { token_type: TokenType::Operator(","), ..}) = comma {
                rst = &rst[1..]; 
            } else {
                break ; 
            } 
        } 
        return Ok((StructDeclaratorList { struct_declarators: ans }, rst)); 
    }
} 

#[derive(Debug, PartialEq, Eq, Clone)] 
pub struct StructDeclarator <'a> {
    pub declarator: Option<Box<Ast<'a>>>,
    pub const_expr: Option<Box<Ast<'a>>>, 
}

impl <'a> Parser<'a> for StructDeclarator<'a> {
    type E = (); 

    fn parse (stack: &mut Vec<Ast<'a>>, tokens: &'a [Token<'a>]) -> Result<(Self, &'a [Token<'a>]), <Self as Parser<'a>>::E> {
        let mut rst = tokens; 
        let declarator = Declarator::parse(stack, rst); 
        let d; 
        match declarator {
            Ok((declarator, r)) => {
                d = Some(Box::new(Ast(AstType::Declarator(declarator), &rst[..rst.len() - r.len()]))); 
                rst = r; 
            }, 
            Err(_) => d = None, 
        } 
        let colon = rst.first(); 
        if let Some(Token { token_type: TokenType::Operator(":"), ..}) = colon {
            rst = &rst[1..]; 
        } else {
            if d.is_none() {
                return Err(()); 
            }
            return Ok((StructDeclarator { declarator: d, const_expr: None }, rst)); 
        }
        let const_expr = ConstExp::parse(stack, rst);
        let c; 
        match const_expr {
            Ok((const_expr, r)) => {
                c = Some(Box::new(Ast(AstType::ConstExp(const_expr), &rst[..rst.len() - r.len()]))); 
                rst = r; 
            }, 
            Err(_) => c = None, 
        } 
        if d.is_some() || c.is_some() {
            return Ok((StructDeclarator { declarator: d, const_expr: c }, rst)); 
        } else {
            return Err(()); 
        } 
    }
}