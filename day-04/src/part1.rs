use glam::IVec2;
use itertools::Itertools;

const DIRECTIONS: [[IVec2; 3]; 8] = [
    [IVec2::new(0, 1), IVec2::new(0, 2), IVec2::new(0, 3)], // Vertical Up
    [IVec2::new(0, -1), IVec2::new(0, -2), IVec2::new(0, -3)], // Vertical Down
    [IVec2::new(1, 1), IVec2::new(2, 2), IVec2::new(3, 3)], // Diagonal Up Right
    [IVec2::new(1, -1), IVec2::new(2, -2), IVec2::new(3, -3)], // Diagonal Down Right
    [IVec2::new(-1, 1), IVec2::new(-2, 2), IVec2::new(-3, 3)], // Diagonal Down Left
    [IVec2::new(-1, -1), IVec2::new(-2, -2), IVec2::new(-3, -3)], // Diagonal Up Left
    [IVec2::new(1, 0), IVec2::new(2, 0), IVec2::new(3, 0)], // Horizontal Right
    [IVec2::new(-1, 0), IVec2::new(-2, 0), IVec2::new(-3, 0)], // Horizontal left
];

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
        .filter_map(|(pos, char)| (char == 'X').then_some(pos))
        .cartesian_product(DIRECTIONS)
        .map(|(pos, dir)| {
            dir.iter()
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
                .join("")
        })
        .filter(|s| s == "MAS")
        .count();
    Ok(format!("{result}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let input = include_str!("../example.txt");
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
