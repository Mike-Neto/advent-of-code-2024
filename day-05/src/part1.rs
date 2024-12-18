use crate::parse;

pub fn process(input: &'static str) -> anyhow::Result<String> {
    let (_, (rules, updates)) = parse(input)?;
    let result: u32 = updates
        .iter()
        .filter_map(|update| {
            update
                .iter()
                .enumerate()
                .all(|(index, page)| {
                    update.iter().skip(index + 1).all(|p| {
                        rules
                            .iter()
                            .any(|(before, after)| before == page && after == p)
                    })
                })
                .then(|| update[update.len() / 2])
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
        assert_eq!("143", process(input)?);
        Ok(())
    }
}
