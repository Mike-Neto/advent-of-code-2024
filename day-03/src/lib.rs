pub mod part1;
pub mod part2;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    multi::{many1, many_till},
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser,
};

#[derive(Debug, PartialEq)]
enum Instruction {
    Do,
    Dont,
    Mul(u64, u64),
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        tag("don't()").map(|_| Instruction::Dont),
        tag("do()").map(|_| Instruction::Do),
        preceded(
            tag("mul"),
            delimited(
                tag("("),
                separated_pair(complete::u64, tag(","), complete::u64)
                    .map(|(a, b)| Instruction::Mul(a, b)),
                tag(")"),
            ),
        ),
    ))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, parse_instruction).map(|(_, instruction)| instruction))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_instruction_works() -> anyhow::Result<()> {
        assert_eq!(("", Instruction::Mul(1, 2)), parse_instruction("mul(1,2)")?);
        Ok(())
    }

    #[test]
    fn parse_works() -> anyhow::Result<()> {
        assert_eq!(("", vec![Instruction::Mul(1, 2)]), parse("mul(1,2)")?);
        assert_eq!(("", vec![Instruction::Mul(1, 2)]), parse("xmul(1,2)")?);
        Ok(())
    }
}
