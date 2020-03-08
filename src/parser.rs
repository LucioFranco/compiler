use crate::{ast::*, Error};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map, recognize},
    error::VerboseError,
};

type IResult<'a, O> = nom::IResult<Input<'a>, O, VerboseError<Input<'a>>>;

type Input<'a> = &'a str;

pub fn parse_module(module: &str) -> IResult<Program<'_>> {
    // let ident = tag
    let lit = all_consuming(lit)(module)?;

    todo!("parser")
}

fn lit(input: Input<'_>) -> IResult<LitExpr> {
    alt((
        map(tag("true"), |_| LitExpr::True),
        map(tag("false"), |_| LitExpr::False),
    ))(input)
}

// fn module(input: Input<'_>) -> IResult<Program<'_>> {
//     todo!("parser:module")
// }

// fn ident(input: Input<'_>) -> IResult<Ident<'_>> {
//     tag()
//     todo!("parser:ident")
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        parse_module("true").unwrap();
    }
}
