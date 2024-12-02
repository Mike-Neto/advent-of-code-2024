use crate::check_report;

pub fn process(input: &str) -> anyhow::Result<String> {
    let reports_count = input
        .lines()
        .filter(|report| {
            let levels: Vec<usize> = report
                .split_whitespace()
                .filter_map(|level| level.parse::<usize>().ok())
                .collect();
            if !check_report(&levels) {
                for index in 0..levels.len() {
                    let mut new_levels = (levels).clone();
                    new_levels.remove(index);
                    if check_report(&new_levels) {
                        return true;
                    } else {
                        continue;
                    }
                }
                return false;
            }
            true
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
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
