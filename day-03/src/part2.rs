use crate::{parse, Instruction};

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
