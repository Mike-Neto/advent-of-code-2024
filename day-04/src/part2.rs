use glam::IVec2;
use itertools::Itertools;

const SAMPLE: [IVec2; 4] = [
    IVec2::new(1, 1),   // Diagonal Up Right
    IVec2::new(1, -1),  // Diagonal Down Right
    IVec2::new(-1, 1),  // Diagonal Down Left
    IVec2::new(-1, -1), // Diagonal Up Left
];
// TODO missing other options
const MATCHES: [&str; 4] = ["SSMM", "MMSS", "MSMS", "SMSM"];

pub fn process(input: &str) -> anyhow::Result<String> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let result = grid
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.into_iter()
                .enumerate()
                .map(|(col, char)| (IVec2::new(col as i32, row as i32), *char))
                .collect::<Vec<(IVec2, char)>>()
        })
        .filter_map(|(pos, char)| {
            if char == 'A' {
                let sample = SAMPLE
                    .iter()
                    .filter_map(|d| {
                        let x: Option<usize> = (pos.x + d.x).try_into().ok();
                        let y: Option<usize> = (pos.y + d.y).try_into().ok();
                        match (x, y) {
                            (Some(x), Some(y)) => match grid.get(y).map(|row| row.get(x)) {
                                Some(Some(char)) => Some(char),
                                _ => None,
                            },
                            _ => None,
                        }
                    })
                    .join("");
                if MATCHES.iter().any(|&m| m == sample) {
                    return Some(pos);
                }
            }
            None
        })
        .count();
    Ok(format!("{result}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let input = include_str!("../example.txt");
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
