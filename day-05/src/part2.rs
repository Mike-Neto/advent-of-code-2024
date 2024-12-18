use crate::parse;

pub fn process(input: &'static str) -> anyhow::Result<String> {
    let (_, (rules, updates)) = parse(input)?;
    let result: u32 = updates
        .iter()
        .filter_map(|update| {
            let valid = update.iter().enumerate().all(|(index, page)| {
                update.iter().skip(index + 1).all(|p| {
                    rules
                        .iter()
                        .any(|(before, after)| before == page && after == p)
                })
            });
            if !valid {
                let mut sorted = update.clone();
                sorted.sort_by(|a, b| {
                    if rules
                        .iter()
                        .any(|(before, after)| before == a && after == b)
                    {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                });
                return Some(sorted[sorted.len() / 2]);
            }
            None
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
        assert_eq!("123", process(input)?);
        Ok(())
    }
}
