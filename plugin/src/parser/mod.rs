use ast;
use errors::*;

mod grammar;

pub fn parse_program(input: &str) -> Result<ast::Program> {
    match grammar::parse_Program(input) {
        Ok(p) => Ok(p),
        Err(_) => bail!("parse error...somewhere...")
    }
}
