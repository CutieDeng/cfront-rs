use std::error::Error;

use crate::{automaton, Keyword};

pub fn process(input: impl IntoIterator<Item = char> ) -> Result<Option<Keyword>, Box<dyn Error>> {
    let mut automaton = automaton::State::default();
    let mut rst = None; 
    for c in input {
        let (a, b) = automaton.read(c).map_err(|_| "")?; 
        automaton = a;  
        rst = b; 
    }
    Ok(rst)
} 

mod correct;
mod incorrect;