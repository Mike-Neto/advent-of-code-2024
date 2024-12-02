use crate::check_report;

pub fn process(input: &str) -> anyhow::Result<String> {
    let reports_count = input
        .lines()
        .filter(|report| {
            let levels: Vec<usize> = report
                .split_whitespace()
                .filter_map(|level| level.parse::<usize>().ok())
                .collect();
            check_report(&levels)
        })
        .count();

    Ok(format!("{reports_count}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let input = include_str!("../example.txt");
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
