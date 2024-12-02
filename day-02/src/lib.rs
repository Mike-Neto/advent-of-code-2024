pub mod part1;
pub mod part2;

pub fn check_report(levels: &Vec<usize>) -> bool {
    let is_descending = levels
        .windows(2)
        .all(|a| a[0] > a[1] && a[0].abs_diff(a[1]) <= 3);
    let is_ascending = levels
        .windows(2)
        .all(|a| a[0] < a[1] && a[0].abs_diff(a[1]) <= 3);
    is_ascending || is_descending
}
