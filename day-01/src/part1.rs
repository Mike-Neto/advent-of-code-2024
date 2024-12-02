pub fn process(input: &str) -> anyhow::Result<String> {
    let lists: Vec<(usize, usize)> = input
        .lines()
        .filter_map(|line| {
            let (left, right) = line.split_once("   ").expect("invalid input");
            Some((
                left.parse().expect("invalid input"),
                right.parse().expect("invalid input"),
            ))
        })
        .collect();
    let mut left: Vec<usize> = lists.iter().cloned().map(|(left, _)| left).collect();
    left.sort();
    let mut right: Vec<usize> = lists.iter().cloned().map(|(_, right)| right).collect();
    right.sort();

    let result: usize = left
        .into_iter()
        .zip(right)
        .map(|(left, right)| left.abs_diff(right))
        .sum();
    Ok(format!("{result}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../example.txt");
        assert_eq!("11", process(input).unwrap());
    }
}
