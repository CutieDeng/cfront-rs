use std::error::Error;

use crate::Keyword;
use crate::tests::process;

#[test]
pub fn align_as_test() -> Result<(), Box<dyn Error>> {
    let input = "alignas"; 
    let output = process(input.chars())?; 
    let output = output.ok_or("")?; 
    assert_eq!(output, Keyword::AlignAs); 
    return Ok(());
}

#[test]
pub fn align_of_test() -> Result<(), Box<dyn Error>> {
    let input = "alignof"; 
    let output = process(input.chars())?; 
    let output = output.ok_or("")?; 
    assert_eq!(output, Keyword::AlignOf); 
    return Ok(());
}

#[test]
pub fn auto_test() -> Result<(), Box<dyn Error>> {
    let input = "auto"; 
    let output = process(input.chars())?; 
    let output = output.ok_or("")?; 
    assert_eq!(output, Keyword::Auto); 
    return Ok(());
}

#[test]
pub fn bool_test() -> Result<(), Box<dyn Error>> {
    let input = "bool"; 
    let output = process(input.chars())?; 
    let output = output.ok_or("")?; 
    assert_eq!(output, Keyword::Bool); 
    return Ok(());
}

#[test]
pub fn break_test() -> Result<(), Box<dyn Error>> {
    let input = "break"; 
    let output = process(input.chars())?; 
    let output = output.ok_or("")?; 
    assert_eq!(output, Keyword::Break); 
    return Ok(());
}

#[test]
pub fn typeof_unqual_test() -> Result<(), Box<dyn Error>> {
    let input = "typeof_unqual"; 
    let output = process(input.chars())?; 
    let output = output.ok_or("")?; 
    assert_eq!(output, Keyword::TypeOfUnqual); 
    return Ok(());
}