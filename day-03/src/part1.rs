use crate::{parse, Instruction};

pub fn process(input: &'static str) -> anyhow::Result<String> {
    let (_, mul_instructions) = parse(input)?;

    let result: u64 = mul_instructions
        .into_iter()
        .filter_map(|instruction| match instruction {
            Instruction::Mul(a, b) => Some(a * b),
            _ => None,
        })
        .sum();

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
}
