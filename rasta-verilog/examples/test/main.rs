use std::fs::File;
use std::io::Write;

use parser::parse;
use rasta_verilog::{generate_verilog, TOP_MODULE};

mod parser;

const CODE: &'static str = "
#[top]
const Machine = fn (arg a: u64,arg b: u64) -> u64 {
    return a+b;
};
";

fn main() {

    let code = generate_verilog(&parse(CODE.into(), "".into())).unwrap();

    let top = TOP_MODULE.lock().unwrap().clone().unwrap();

    writeln!(File::create(top+".v").unwrap(),"{}",code).unwrap();
}
