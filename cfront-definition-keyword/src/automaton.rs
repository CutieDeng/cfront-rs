use crate::Keyword;

#[allow(non_camel_case_types)] 
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum State {
    #[default]
    Empty, 
    A, 
    ALign(usize), 
    AlignAs(usize), 
    AlignOf(usize), 
    AUto(usize), 
    ASm(usize), 
    B, 
    BOol (usize), 
    BReak(usize), 
    C, 
    CAse(usize), 
    CHar(usize), 
    COn(usize), 
    ConSt(usize), 
    ConstExpr(usize), 
    ConTinue(usize), 
    D, 
    DEfault(usize), 
    DOuble(usize), 
    E, 
    ELse(usize), 
    ENum(usize), 
    EXtern(usize), 
    F, 
    FAlse(usize),
    FLoat(usize), 
    FOr(usize), 
    Goto(usize), 
    I, 
    IF, 
    IN, 
    InLine(usize), 
    InT, 
    Long(usize), 
    Nullptr(usize), 
    Re(usize), 
    ReGister(usize), 
    ReStrict(usize), 
    ReTurn(usize), 
    S, 
    SHort(usize), 
    SI, 
    SiGned(usize), 
    SiZeof(usize), 
    ST, 
    StAtic(usize), 
    Static_Assert(usize), 
    StRuct(usize), 
    SWitch(usize), 
    T, 
    THread_local(usize), 
    TRue(usize), 
    TYpe(usize), 
    TypeDef(usize), 
    TypeOf(usize), 
    TypeOf_Unqual(usize),
    Un(usize), 
    UnIon(usize), 
    UnSigned(usize), 
    Vo(usize), 
    VoId(usize), 
    VoLatile(usize), 
    While(usize), 
    /// '_' 
    Z,
    ZA, 
    ZALign(usize), 
    ZAlignAs(usize), 
    ZAlignOf(usize), 
    ZATomic(usize), 
    ZBool(usize), 
    ZComplex(usize), 
    ZDecimal(usize), 
    ZDecimal32(usize), 
    ZDecimal64(usize), 
    ZDecimal128(usize), 
    ZGeneric(usize), 
    ZImaginary(usize), 
    ZNoreturn(usize), 
    ZStaticassert(usize), 
    ZThreadlocal(usize), 
}

impl State {
    pub fn read(self, c: char) -> Result<(Self, Option<Keyword>), ()> {
        use State::*; 
        let keyword_consist = c.is_ascii_alphabetic() || c == '_';
        if !keyword_consist {
            return Err(());
        }
        let ans = match self {
            Empty => {
                match c {
                    'a' => (A, None), 
                    'b' => (B, None), 
                    'c' => (C, None), 
                    'd' => (D, None), 
                    'e' => (E, None), 
                    'f' => (F, None), 
                    'g' => (Goto(1), None), 
                    'i' => (I, None), 
                    'l' => (Long(1), None), 
                    'n' => (Nullptr(1), None), 
                    'r' => (Re(1), None), 
                    's' => (S, None), 
                    't' => (T, None), 
                    'u' => (Un(1), None), 
                    'v' => (Vo(1), None), 
                    'w' => (While(1), None), 
                    '_' => (Z, None), 
                    _ => return Err(()), 
                } 
            }
            A => {
                match c {
                    'l' => (ALign(1), None), 
                    's' => (ASm(1), None), 
                    'u' => (AUto(1), None), 
                    _ => return Err(()), 
                }  
            }
            ALign(i) => {
                let l = "lign"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {
                            (ALign(i + 1), None) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => {
                        match c {
                            'a' => (AlignAs(1), None), 
                            'o' => (AlignOf(1), None), 
                            _ => return Err(()), 
                        }
                    },
                }
            }
            AlignAs(1) => {
                match c {
                    's' => (AlignAs(2), Some(Keyword::AlignAs)), 
                    _ => return Err(()), 
                } 
            }
            AlignOf(1) => {
                match c {
                    'f' => (AlignOf(2), Some(Keyword::AlignOf)), 
                    _ => return Err(()), 
                }  
            }
            AUto(i) => {
                let l = "uto"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::Auto); 
                            } else {
                                ret = None; 
                            }
                            (AUto(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                }
            }
            ASm(1) => {
                match c {
                    'm' => (ASm(2), Some(Keyword::Asm)), 
                    _ => return Err(()), 
                }  
            }
            B => {
                match c {
                    'o' => (BOol(1), None), 
                    'r' => (BReak(1), None), 
                    _ => return Err(()), 
                }   
            }
            BOol(i) => {
                let l = "ool"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::Bool); 
                            } else {
                                ret = None; 
                            }
                            (BOol(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                }
            }
            BReak(i) => {
                let l = "reak"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::Break); 
                            } else {
                                ret = None; 
                            }
                            (BReak(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                } 
            }
            C => {
                match c {
                    'a' => (CAse(1), None), 
                    'h' => (CHar(1), None), 
                    'o' => (COn(1), None), 
                    's' => (ConSt(1), None), 
                    _ => return Err(()), 
                }  
            }
            CAse(i) => {
                let l = "ase"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::Case); 
                            } else {
                                ret = None; 
                            }
                            (CAse(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                }  
            }
            CHar(i) => {
                let l = "har"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::Char); 
                            } else {
                                ret = None; 
                            }
                            (CHar(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                }
            }
            COn(1) => {
                match c {
                    'n' => (COn(2), None), 
                    _ => return Err(()), 
                }   
            }
            COn(2) => {
                match c {
                    's' => (ConSt(1), None), 
                    _ => return Err(()), 
                } 
            }
            ConSt(1) => {
                match c {
                    't' => (ConSt(2), Some(Keyword::Const)), 
                    _ => return Err(()), 
                } 
            }
            ConSt(2) => {
                match c {
                    'e' => (ConstExpr(1), None), 
                    _ => return Err(()), 
                }  
            }
            ConstExpr(i) => {
                let l = "expr"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::Constexpr); 
                            } else {
                                ret = None; 
                            } 
                            (ConstExpr(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                } 
            }
            ConTinue(i) => {
                let l = "tinue"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::Continue); 
                            } else {
                                ret = None; 
                            }
                            (ConTinue(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                } 
            }
            D => {
                match c {
                    'e' => (DEfault(1), None), 
                    'o' => (DOuble(1), None), 
                    _ => return Err(()), 
                }   
            }
            DEfault(i) => {
                let l = "efault"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 { 
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::Default); 
                            } else {
                                ret = None; 
                            }
                            (DEfault(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                }  
            }
            DOuble(i) => {
                let l = "ouble"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 { 
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::Double); 
                            } else {
                                ret = None; 
                            }
                            (DOuble(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                } 
            }
            E => {
                match c {
                    'l' => (ELse(1), None), 
                    'n' => (ENum(1), None), 
                    'x' => (EXtern(1), None), 
                    _ => return Err(()),  
                }
            }
            ELse(i) => {
                let l = "lse"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 { 
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::Else); 
                            } else {
                                ret = None; 
                            }
                            (ELse(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                } 
            }
            ENum(i) => {
                let l = "num"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 { 
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::Enum); 
                            } else {
                                ret = None; 
                            }
                            (ENum(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                }  
            }
            EXtern(i) => {
                let l = "xtern"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 { 
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::Extern); 
                            } else {
                                ret = None; 
                            }
                            (EXtern(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                }
            }
            F => {
                match c {
                    'a' => (FAlse(1), None), 
                    'l' => (FLoat(1), None), 
                    'o' => (FOr(1), None), 
                    _ => return Err(()), 
                }   
            }
            FAlse(i) => {
                let l = "alse"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 { 
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::False); 
                            } else {
                                ret = None; 
                            }
                            (FAlse(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                } 
            }
            FLoat(i) => {
                let l = "loat"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 { 
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::Float); 
                            } else {
                                ret = None; 
                            }
                            (FLoat(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                }  
            }
            FOr(i) => {
                let l = "or"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 { 
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::For); 
                            } else {
                                ret = None; 
                            }
                            (FOr(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                }
            }
            Goto(i) => {
                let l = "oto"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 { 
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::Goto); 
                            } else {
                                ret = None; 
                            }
                            (Goto(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                }  
            }
            I => {
                match c {
                    'f' => (IF, Some(Keyword::If)), 
                    'n' => (IN, None), 
                    _ => return Err(()), 
                }   
            }
            IN => {
                match c {
                    'l' => (InLine(1), None), 
                    't' => (InT, Some(Keyword::Int)), 
                    _ => return Err(()), 
                }   
            }
            InLine(i) => {
                let l = "line"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 { 
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::Inline); 
                            } else {
                                ret = None; 
                            }
                            (InLine(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                } 
            }
            Long(i) => {
                let l = "long"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 { 
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::Long); 
                            } else {
                                ret = None; 
                            }
                            (Long(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                }  
            }
            Nullptr(i) => {
                let l = "nullptr";
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::Nullptr); 
                            } else {
                                ret = None; 
                            }
                            (Nullptr(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                } 
            }
            Re(1) => {
                if c == 'e' {
                    (Re(2), None) 
                } else {
                    return Err(());  
                }
            }
            Re(2) => {
                match c {
                    'g' => (ReGister(1), None), 
                    's' => (ReStrict(1), None), 
                    't' => (ReTurn(1), None), 
                    _ => return Err(()),  
                }
            }
            ReGister(i) => {
                let l = "gister"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 { 
                            (ReGister(i + 1), None) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => {
                        match c {
                            'e' => (ReStrict(1), None), 
                            't' => (ReTurn(1), None), 
                            _ => return Err(()), 
                        }
                    },
                } 
            }
            ReStrict(i) => {
                let l = "strict"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 { 
                            (ReStrict(i + 1), None) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => {
                        match c {
                            't' => (ReTurn(1), None), 
                            _ => return Err(()), 
                        }
                    },
                }  
            }
            ReTurn(i) => {
                let l = "turn"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 { 
                            (ReTurn(i + 1), None) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                }  
            }
            S => {
                match c {
                    'h' => (SHort(1), None), 
                    'i' => (SI, None), 
                    't' => (ST, None), 
                    _ => return Err(()), 
                }    
            }
            SHort(i) => {
                let l = "hort"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {  
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::Short); 
                            } else {
                                ret = None; 
                            }
                            (SHort(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                } 
            }
            SI => {
                match c {
                    'g' => (SiGned(1), None), 
                    'z' => (SiZeof(1), None), 
                    _ => return Err(()), 
                }    
            }
            SiGned(i) => {
                let l = "gned"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {  
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::Signed); 
                            } else {
                                ret = None; 
                            }
                            (SiGned(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => {
                        match c {
                            'e' => (SiZeof(1), None), 
                            _ => return Err(()), 
                        }
                    },
                }  
            }
            SiZeof(i) => {
                let l = "zeof"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {  
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::Sizeof); 
                            } else {
                                ret = None; 
                            }
                            (SiZeof(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                }  
            }
            ST => {
                match c {
                    'a' => (StAtic(1), None), 
                    'r' => (StRuct(1), None), 
                    _ => return Err(()), 
                } 
            }
            StAtic(i) => {
                let l = "atic"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {  
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::Static); 
                            } else {
                                ret = None; 
                            }
                            (StAtic(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => {
                        match c {
                            '_' => (Static_Assert(1), None), 
                            _ => return Err(()), 
                        }
                    },
                }  
            }
            Static_Assert(i) => {
                let l = "_assert"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {  
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::StaticAssert); 
                            } else {
                                ret = None; 
                            }
                            (Static_Assert(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                }   
            }
            StRuct(i) => {
                let l = "ruct"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {  
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::Struct); 
                            } else {
                                ret = None; 
                            }
                            (StRuct(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                } 
            }
            SWitch(i) => {
                let l = "witch"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {  
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::Switch); 
                            } else {
                                ret = None; 
                            }
                            (SWitch(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                }
            }
            T => {
                match c {
                    'h' => (THread_local(1), None), 
                    'r' => (TRue(1), None), 
                    'y' => (TYpe(1), None), 
                    _ => return Err(()), 
                }    
            }
            THread_local(i) => {
                let l = "hread_local"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {  
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::ThreadLocal); 
                            } else {
                                ret = None; 
                            }
                            (THread_local(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                }  
            }
            TRue(i) => {
                let l = "rue"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {  
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::True); 
                            } else {
                                ret = None; 
                            }
                            (TRue(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                } 
            }
            TYpe(i) => {
                let l = "ype"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {  
                            (TYpe(i + 1), None) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => {
                        match c {
                            'd' => (TypeDef(1), None), 
                            'o' => (TypeOf(1), None), 
                            _ => return Err(()), 
                        }
                    },
                }  
            }
            TypeDef(i) => {
                let l = "def"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {  
                            (TypeDef(i + 1), None) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => {
                        match c {
                            'i' => (TypeOf(1), None), 
                            _ => return Err(()), 
                        }
                    },
                }   
            }
            TypeOf(1) => {
                match c {
                    'f' => (TypeOf(2), Some(Keyword::TypeOf)), 
                    _ => return Err(()), 
                }
            }
            TypeOf(2) => {
                match c {
                    '_' => (TypeOf_Unqual(1), None), 
                    _ => return Err(()), 
                } 
            }
            TypeOf_Unqual(i) => {
                let l = "_unqual"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {  
                            let ret ;
                            if i + 1 == l.len() {
                                ret = Some(Keyword::TypeOfUnqual); 
                            } else {
                                ret = None; 
                            } 
                            (TypeOf_Unqual(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                } 
            }
            Un(1) => {
                match c {
                    'n' => (Un(2), None), 
                    _ => return Err(()), 
                }
            }
            Un(2) => {
                match c {
                    'i' => (UnIon(1), None), 
                    's' => (UnSigned(1), None), 
                    _ => return Err(()),  
                }
            }
            UnIon(i) => {
                let l = "ion"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {  
                            (UnIon(i + 1), None) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => {
                        match c {
                            's' => (UnSigned(1), None), 
                            _ => return Err(()), 
                        }
                    },
                }  
            }
            UnSigned(i) => {
                let l = "signed"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {  
                            (UnSigned(i + 1), None) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                }  
            }
            Vo(1) => {
                if c == 'o' {
                    (Vo(2), None) 
                } else {
                    return Err(());   
                }
            }
            Vo(2) => {
                match c {
                    'i' => (VoId(1), None), 
                    'l' => (VoLatile(1), None), 
                    _ => return Err(()),  
                } 
            }
            VoId(1) => {
                match c {
                    'd' => (VoId(2), Some(Keyword::Void)), 
                    _ => return Err(()), 
                }   
            }
            VoLatile(i) => {
                let l = "latile"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {  
                            (VoLatile(i + 1), None) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                }   
            }
            While(i) => {
                let l = "while"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {  
                            (While(i + 1), None) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                } 
            }
            Z => {
                match c {
                    'A' => (ZA, None), 
                    'B' => (ZBool(1), None), 
                    'C' => (ZComplex(1), None), 
                    'D' => (ZDecimal(1), None),     
                    'G' => (ZGeneric(1), None), 
                    'I' => (ZImaginary(1), None), 
                    'N' => (ZNoreturn(1), None), 
                    'S' => (ZStaticassert(1), None), 
                    'T' => (ZThreadlocal(1), None), 
                    _ => return Err(()), 
                }
            }
            ZA => {
                match c {
                    'l' => (ZALign(1), None), 
                    't' => (ZATomic(1), None), 
                    _ => return Err(()), 
                } 
            }
            ZALign(i) => {
                let l = "lign"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {
                            (ZALign(i + 1), None) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => {
                        match c {
                            'a' => (ZAlignAs(1), None), 
                            'o' => (ZAlignOf(1), None), 
                            _ => return Err(()), 
                        }
                    },
                } 
            }
            ZAlignAs(1) => {
                match c {
                    's' => (ZAlignAs(2), Some(Keyword::_AlignAs)), 
                    _ => return Err(()), 
                }  
            }
            ZAlignOf(1) => {
                match c {
                    'f' => (ZAlignOf(2), Some(Keyword::_AlignOf)), 
                    _ => return Err(()), 
                }   
            }
            ZATomic(i) => {
                let l = "tomic"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {
                            let ret ; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::_Atomic); 
                            } else {
                                ret = None; 
                            } 
                            (ZATomic(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                } 
            }
            ZBool(i) => {
                let l = "bool"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {
                            let ret ; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::_Bool); 
                            } else {
                                ret = None; 
                            } 
                            (ZBool(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                }   
            }
            ZComplex(i) => {
                let l = "complex"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {
                            let ret ; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::_Complex); 
                            } else {
                                ret = None; 
                            } 
                            (ZComplex(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                }  
            }
            ZDecimal(i) => {
                let l = "Decimal"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {
                            (ZDecimal(i + 1), None) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => {
                        match c {
                            '3' => (ZDecimal32(1), None), 
                            '6' => (ZDecimal64(1), None), 
                            '1' => (ZDecimal128(1), None), 
                            _ => return Err(()), 
                        }
                    }, 
                }
            }
            ZDecimal32(1) => {
                match c {
                    '2' => (ZDecimal32(2), Some(Keyword::_Decimal32)), 
                    _ => return Err(()), 
                }
            }
            ZDecimal64(1) => {
                match c {
                    '4' => (ZDecimal64(2), Some(Keyword::_Decimal64)), 
                    _ => return Err(()),  
                }
            }
            ZDecimal128(1) => {
                match c {
                    '8' => (ZDecimal128(2), None), 
                    _ => return Err(()),  
                } 
            }
            ZDecimal128(2) => {
                match c {
                    '8' => (ZDecimal128(3), Some(Keyword::_Decimal128)), 
                    _ => return Err(()),  
                }  
            }
            ZGeneric(i) => {
                let l = "Generic"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::_Generic); 
                            } else {
                                ret = None; 
                            } 
                            (ZGeneric(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                } 
            }
            ZImaginary(i) => {
                let l = "Imaginary"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::_Imaginary); 
                            } else {
                                ret = None; 
                            } 
                            (ZImaginary(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()),
                } 
            }
            ZNoreturn(i) => {
                let l = "Noreturn";
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::_Noreturn); 
                            } else {
                                ret = None; 
                            } 
                            (ZNoreturn(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()), 
                }
            }
            ZStaticassert(i) => {
                let l = "Static_assert";
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::_StaticAssert); 
                            } else {
                                ret = None; 
                            } 
                            (ZStaticassert(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()),  
                } 
            }
            ZThreadlocal(i) => {
                let l = "Thread_local"; 
                let c2 = l.chars().nth(i); 
                match c2 {
                    Some(c2) => {
                        if c == c2 {
                            let ret; 
                            if i + 1 == l.len() {
                                ret = Some(Keyword::_ThreadLocal); 
                            } else {
                                ret = None; 
                            } 
                            (ZThreadlocal(i + 1), ret) 
                        } else {
                            return Err(()); 
                        }
                    },
                    None => 
                        return Err(()),  
                } 
            }
            _ => 
                return Err(()), 
        }; 
        return Ok(ans); 
    }
}
