use nom::{
    bytes::complete::tag,
    character::complete::{self, multispace1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub mod part1;
pub mod part2;

fn parse_rules(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    separated_list1(
        newline,
        separated_pair(complete::u32, tag("|"), complete::u32),
    )(input)
}
fn parse_updates(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(newline, separated_list1(tag(","), complete::u32))(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<(u32, u32)>, Vec<Vec<u32>>)> {
    separated_pair(parse_rules, multispace1, parse_updates)(input)
}
