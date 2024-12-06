use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar},
    multi::{many1, many_till},
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser,
};

fn parse_instruction(input: &str) -> IResult<&str, (u64, u64)> {
    preceded(
        tag("mul"),
        delimited(
            tag("("),
            separated_pair(complete::u64, tag(","), complete::u64),
            tag(")"),
        ),
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    many1(many_till(anychar, parse_instruction).map(|(_, instruction)| instruction))(input)
}

pub fn process(input: &'static str) -> anyhow::Result<String> {
    let (_, mul_instructions) = parse(input)?;

    let result: u64 = mul_instructions.into_iter().map(|(a, b)| a * b).sum();

    Ok(format!("{result}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let input = include_str!("../example.txt");
        assert_eq!("161", process(input)?);
        Ok(())
    }

    #[test]
    fn parse_instruction_works() -> anyhow::Result<()> {
        assert_eq!(("", (1, 2)), parse_instruction("mul(1,2)")?);
        Ok(())
    }

    #[test]
    fn parse_works() -> anyhow::Result<()> {
        assert_eq!(("", vec![(1, 2)]), parse("mul(1,2)")?);
        assert_eq!(("", vec![(1, 2)]), parse("xmul(1,2)")?);
        Ok(())
    }
}
