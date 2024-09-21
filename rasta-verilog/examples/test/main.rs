use std::{env::args, fs::File};
use std::io::{Read, Write};

use parser::parse;
use rasta_verilog::{generate_verilog, TOP_MODULE};

mod parser;

fn main() {

    let mut args = args();

    if args.len() != 2 {
        eprintln!("Usage: {} <source_file>", args.next().unwrap());
        std::process::exit(1);
    }

    let _exe = args.nth(0).unwrap();
    let source_file_path = args.nth(0).unwrap();

    let mut source_file = File::open(source_file_path.clone()).unwrap();
    let mut source = String::new();
    source_file.read_to_string(&mut source).unwrap();

    let ast = parse(source, source_file_path);

    let code = generate_verilog(&ast).unwrap();

    let top = TOP_MODULE.lock().unwrap().clone().unwrap();

    writeln!(File::create(top+".v").unwrap(),"{}",code).unwrap();
}
