use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    multi::{many1, many_till},
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser,
};

#[derive(Debug)]
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

pub fn process(input: &'static str) -> anyhow::Result<String> {
    let (_, mul_instructions) = parse(input)?;

    let result: u64 = mul_instructions
        .into_iter()
        .fold(
            (vec![], true),
            |mut acc: (Vec<(u64, u64)>, bool), instruction| {
                match instruction {
                    Instruction::Do => acc.1 = true,
                    Instruction::Dont => acc.1 = false,
                    Instruction::Mul(a, b) => {
                        if acc.1 {
                            acc.0.push((a, b));
                        }
                    }
                }

                return acc;
            },
        )
        .0
        .into_iter()
        .map(|(a, b)| a * b)
        .sum();

    Ok(format!("{result}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let input = include_str!("../example2.txt");
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
