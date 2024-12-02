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
    let left: Vec<usize> = lists.iter().cloned().map(|(left, _)| left).collect();
    let right: Vec<usize> = lists.iter().cloned().map(|(_, right)| right).collect();

    let result: usize = left
        .into_iter()
        .map(|value| {
            let count = right.iter().cloned().filter(|&r| r == value).count();
            value * count
        })
        .sum();
    Ok(format!("{result}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../example.txt");
        assert_eq!("31", process(input).unwrap());
    }
}
