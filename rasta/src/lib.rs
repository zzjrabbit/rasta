mod defs;
mod expr;
mod program;
mod stmt;
mod types;

pub use defs::*;
pub use expr::*;
pub use program::*;
pub use stmt::*;
pub use types::*;

use serde_derive::*;

/// Serialize the given AST
pub fn serialize(unit: &CompUnit) -> Vec<u8> {
    rmp_serde::to_vec(unit).unwrap()
}

/// Desialize the given AST
pub fn deserialize(data: Vec<u8>) -> CompUnit {
    rmp_serde::from_slice(&data).unwrap()
}

/// The span of the code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    start: (usize,usize),
    end: (usize,usize),
    string: String,
    file: String,
}

impl Span {
    pub fn new(start: (usize,usize), end: (usize,usize), code: String, file: String) -> Self {
        Self {
            start,
            end,
            string: code,
            file,
        }
    }
}

impl core::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"\x1b[1;34m---> \x1b[0m{}:{}:{}\x1b[1;34m\n",self.file,self.start.0,self.start.1)?;

        let num_len = format!("{}", self.start.0).len();
        
        for _ in 0..=num_len {
            write!(f, " ")?;
        }
        write!(f,"|\n")?;

        write!(f, "{} | \x1b[0m{}\x1b[1;34m", self.start.0,self.string)?;

        for _ in 0..=num_len {
            write!(f, " ")?;
        }
        write!(f,"| ")?;

        for _ in 0..self.start.1-1 {
            write!(f, " ")?;
        }

        for _ in self.start.1..self.end.1 {
            write!(f, "\x1b[1;31m^\x1b[1;34m")?;
        }
        Ok(())
    }
    
}


