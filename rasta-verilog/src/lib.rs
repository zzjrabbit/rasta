use std::{cell::RefCell, rc::Rc};
use std::fmt::Write;

use rasta::*;

mod gen;

pub use gen::*;

#[derive(Debug)]
pub enum ErrorKind {

}

#[derive(Debug)]
pub struct Error(pub ErrorKind, pub String);



pub fn generate_verilog(ast: &CompUnit) -> Result<String, Error> {
    let code = Rc::new(RefCell::new(String::new()));
    ast.generate(code.clone())?;
    let code = code.borrow().clone();
    
    Ok(code)
}
