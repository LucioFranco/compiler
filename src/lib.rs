mod ast;
mod parser;

use parser::parse_module;

pub type Error = anyhow::Error;

pub fn compile(code: &str) -> Result<(), Error> {
    let _mod = parse_module(code).unwrap();

    Ok(())
}
