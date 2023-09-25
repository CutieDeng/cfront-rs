use std::error::Error;

use crate::Keyword;
use crate::tests::process;

#[test]
pub fn align_as_test0() -> Result<(), Box<dyn Error>> {
    let input = "_alignas"; 
    let output = process(input.chars()); 
    assert!(output.is_err());
    return Ok(());
}

#[test]
pub fn align_as_test1() -> Result<(), Box<dyn Error>> {
    let input = "aligna"; 
    let output = process(input.chars())?; 
    assert!(output.is_none()); 
    return Ok(());
}