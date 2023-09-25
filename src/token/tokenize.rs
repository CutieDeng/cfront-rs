use super::{Token, TokenType};

pub fn tokenize(input: &str) -> Vec<Token<'_>> {
    let char_indices: Vec<_> = input.char_indices().collect(); 
    let char_indices = &char_indices[..]; 
    let mut idx = 0; 
    let (mut last_line, mut last_column) = (0, 0); 
    let (mut line, mut column) = (0, 0); 
    let mut just_ignore: usize = 0; 
    enum CommentState {
        None, 
        Line, 
        Block,
    }
    let mut comment_state = CommentState::None; 
    enum QuotingState {
        None, 
        Single(usize),
        Double(usize),
    }
    let mut quoting_state = QuotingState::None; 
    let mut lidx: Option<usize> = None; 
    enum NormalState<'a> {
        Identifier, 
        Number(Option<&'a str>), 
    }
    let mut normal_state = NormalState::Identifier; 
    let mut ans = Vec::new(); 
    while let Some((i, c)) = char_indices.get(idx) { 
        'scope : {
            match just_ignore {
                ref mut x if *x > 0 => {
                    *x -= 1; 
                    break 'scope; 
                } 
                _ => (), 
            }
            match quoting_state {
                QuotingState::None => {
                    if *c == '\'' {
                        quoting_state = QuotingState::Single(*i); 
                        break 'scope; 
                    } else if *c == '"' {
                        quoting_state = QuotingState::Double(*i); 
                        break 'scope; 
                    } 
                }
                QuotingState::Single(l) => {
                    if *c == '\'' {
                        ans.push(Token { token_type: TokenType::CharLiteral(&input[l+1..*i], true), line, column, }); 
                        quoting_state = QuotingState::None; 
                    } else if *c == '\n' {
                        ans.push(Token { token_type: TokenType::CharLiteral(&input[l+1..*i], false), line, column, }); 
                        quoting_state = QuotingState::None; 
                    }
                    break 'scope; 
                }
                QuotingState::Double(l) => {
                    if *c == '"' {
                        ans.push(Token { token_type: TokenType::StringLiteral(&input[l+1..*i], true), line, column, }); 
                        quoting_state = QuotingState::None; 
                    } else if *c == '\n' {
                        ans.push(Token { token_type: TokenType::StringLiteral(&input[l+1..*i], false), line, column, }); 
                        quoting_state = QuotingState::None; 
                    } 
                    break 'scope; 
                }
            }
            match comment_state {
                CommentState::None => {
                    if *c == '/' {
                        if let Some((_, c2)) = char_indices.get(idx + 1) {
                            if *c2 == '/' {
                                comment_state = CommentState::Line; 
                                just_ignore = 1; 
                                break 'scope; 
                            } else if *c2 == '*' {
                                comment_state = CommentState::Block; 
                                just_ignore = 1; 
                                break 'scope; 
                            }
                        } 
                    }
                }
                CommentState::Line => {
                    if *c == '\n' {
                        comment_state = CommentState::None; 
                    }
                    break 'scope;  
                }
                CommentState::Block => {
                    if *c == '*' {
                        if let Some((_, c2)) = char_indices.get(idx + 1) {
                            if *c2 == '/' {
                                comment_state = CommentState::None; 
                                just_ignore = 1; 
                                break 'scope; 
                            }
                        } 
                    } 
                    break 'scope;
                }
            } 
            let mut punt = false; 
            match lidx {
                Some(l) => {
                    let mut should_put = false; 
                    if c.is_ascii_whitespace() {
                        should_put = true; 
                    }
                    if c.is_ascii_punctuation() {
                        let mut i = true; 
                        if *c == '.' { 
                            if let NormalState::Number(_) = normal_state {
                                i = false; 
                            } 
                        }
                        if *c == '$' || *c == '@' || *c == '_' { 
                            i = false; 
                        }
                        if i {
                            should_put = true; 
                            punt = true; 
                        }
                    } 
                    if should_put {
                        match normal_state {
                            NormalState::Identifier => {
                                let s = &input[l..*i]; 
                                if let Some(k) = try_into_keyword(s) {
                                    ans.push(Token { token_type: TokenType::Keyword(k), line: last_line, column: last_column, }); 
                                } else {
                                    ans.push(Token { token_type: TokenType::Identifier(s), line: last_line, column: last_column, }); 
                                } 
                            }
                            NormalState::Number(prefix) => {
                                let s = &input[l..*i]; 
                                ans.push(Token { token_type: TokenType::NumberLiteral(s, prefix), line: last_line, column: last_column }); 
                            }
                        }
                        lidx = None;
                    }
                },
                None => {
                    if c.is_whitespace() {
                        break 'scope;  
                    }
                    if c.is_ascii_punctuation() {
                        punt = true; 
                        if *c == '$' || *c == '@' || *c == '_' { 
                            punt = false;
                        }
                    } 
                    if !punt {
                        lidx = Some(*i); 
                        if c.is_digit(10) {
                            normal_state = NormalState::Number(None); 
                            if *c == '0' {
                                let p = char_indices.get(idx + 1); 
                                match p {
                                    Some((_, 'x')) | Some((_, 'X')) | Some((_, 'b')) | Some((_, 'B')) => {
                                        normal_state = NormalState::Number(Some(&input[*i..i+2])); 
                                        just_ignore = 1; 
                                        break 'scope; 
                                    }
                                    _ => {
                                        normal_state = NormalState::Number(Some(&input[*i..i+1])); 
                                    }
                                } 
                            }
                        } else {
                            normal_state = NormalState::Identifier; 
                        }
                    }
                },
            }
            if punt {
                use TokenType::*; 
                match c {
                    '(' => {
                        ans.push(Token { token_type: Parenthesis { is_left: true }, line, column, }); 
                    }
                    ')' => {
                        ans.push(Token { token_type: Parenthesis { is_left: false }, line, column, }); 
                    } 
                    '[' => {
                        ans.push(Token { token_type: Bracket { is_left: true }, line, column, }); 
                    } 
                    ']' => {
                        ans.push(Token { token_type: Bracket { is_left: false }, line, column, }); 
                    } 
                    '{' => {
                        ans.push(Token { token_type: Brace { is_left: true }, line, column, }); 
                    } 
                    '}' => {
                        ans.push(Token { token_type: Brace { is_left: false }, line, column, }); 
                    } 
                    | '.' | ',' | ';' | '~' | ':' => {
                        ans.push(Token { token_type: Operator(&input[*i..i+1]), line, column, });  
                    } 
                    // every puntc here can be followed by '=' 
                    | '<' | '=' | '>' | '+' | '-' | '*' | '/' | '%' | '&' | '^' | '|' | '!' => {
                        if *c == '<' {
                            let p = (char_indices.get(idx + 1), char_indices.get(idx + 2)); 
                            match p {
                                (Some((_, '<')), Some((_, '='))) => {
                                    ans.push(Token { token_type: Operator(&input[*i..i+3]), line, column: column + 2 });
                                    just_ignore = 2; 
                                    break 'scope; 
                                }
                                _ => (), 
                            }
                        }        
                        if *c == '>' {
                            let p = (char_indices.get(idx + 1), char_indices.get(idx + 2)); 
                            match p {
                                (Some((_, '>')), Some((_, '='))) => {
                                    ans.push(Token { token_type: Operator(&input[*i..i+3]), line, column: column + 2 });
                                    just_ignore = 2; 
                                    break 'scope; 
                                }
                                _ => (), 
                            } 
                        }
                        let p = char_indices.get(idx + 1);
                        match p {
                            Some((_, '=')) => {
                                ans.push(Token { token_type: Operator(&input[*i..i+2]), line, column: column + 1 });
                                just_ignore = 1; 
                                println!("here");
                                break 'scope;  
                            }
                            _ => (), 
                        }
                        if *c == '+' || *c == '-' || *c == '&' || *c == '|' {
                            match p {
                                Some((_, b)) if *b == *c => {
                                    ans.push(Token { token_type: Operator(&input[*i..i+2]), line, column: column + 1});
                                    just_ignore = 1; 
                                    break 'scope;  
                                }
                                _ => (),  
                            }
                        }
                        if *c == '-' {
                            match p {
                                Some((_, '>')) => {
                                    ans.push(Token { token_type: Operator(&input[*i..i+2]), line, column: column + 1});
                                    just_ignore = 1; 
                                    break 'scope;  
                                }
                                _ => (),  
                            }
                        } 
                        ans.push(Token { token_type: Operator(&input[*i..i+1]), line, column }); 
                    }
                    _ => {
                        ans.push(Token { token_type: Operator(&input[*i..i+1]), line, column }); 
                    }
                }
            }
        }
        (last_line, last_column) = (line, column); 
        if *c == '\n' {
            line += 1; 
            column = 0; 
        } else {
            column += 1; 
        } 
        idx += 1; 
    }
    return ans; 
}

pub fn try_into_keyword(input: &str) -> Option<super::Keyword> {
    use super::Keyword::*; 
    let ans = match input {
        "alignas" => AlignAs, 
        "alignof" => AlignOf, 
        "auto" => Auto, 
        "bool" => Bool, 
        "break" => Break, 
        "case" => Case, 
        "char" => Char, 
        "const" => Const, 
        "constexpr" => Constexpr, 
        "continue" => Continue, 
        "default" => Default, 
        "do" => Do, 
        "double" => Double, 
        "else" => Else, 
        "enum" => Enum, 
        "extern" => Extern, 
        "false" => False, 
        "float" => Float, 
        "for" => For, 
        "goto" => Goto, 
        "if" => If, 
        "inline" => Inline, 
        "int" => Int, 
        "long" => Long, 
        "nullptr" => Nullptr, 
        "register" => Register, 
        "restrict" => Restrict, 
        "return" => Return, 
        "short" => Short, 
        "signed" => Signed, 
        "sizeof" => Sizeof, 
        "static" => Static, 
        "static_assert" => StaticAssert, 
        "struct" => Struct, 
        "switch" => Switch, 
        "thread_local" => ThreadLocal, 
        "true" => True, 
        "typedef" => Typedef, 
        "typeof" => TypeOf, 
        "typeof_unqual" => TypeOfUnqual, 
        "union" => Union, 
        "unsigned" => Unsigned, 
        "void" => Void, 
        "volatile" => Volatile, 
        "while" => While, 
        "_Alignas" => _AlignAs, 
        "_Alignof" => _AlignOf, 
        "_Atomic" => _Atomic, 
        "_Bool" => _Bool, 
        "_Complex" => _Complex, 
        "_Decimal128" => _Decimal128, 
        "_Decimal32" => _Decimal32, 
        "_Decimal64" => _Decimal64, 
        "_Generic" => _Generic, 
        "_Imaginary" => _Imaginary, 
        "_Noreturn" => _Noreturn, 
        "_Static_assert" => _StaticAssert, 
        "_Thread_local" => _ThreadLocal, 
        "asm" => Asm, 
        _ => return None, 
    }; 
    Some(ans) 
}