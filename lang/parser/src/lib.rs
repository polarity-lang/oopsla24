pub mod cst;
mod grammar;
mod result;

use std::rc::Rc;

use grammar::cst::{ExpParser, PrgParser};
pub use result::*;

pub fn parse_exp(s: &str) -> Result<Rc<cst::Exp>, ParseError> {
    ExpParser::new().parse(s).map_err(From::from)
}

pub fn parse_program(s: &str) -> Result<cst::Prg, ParseError> {
    PrgParser::new().parse(s).map_err(From::from)
}
