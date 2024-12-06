use day_03::part1::process;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../data.txt");
    let result = process(file)?;
    println!("{}", result);
    assert_eq!("173731097", result);
    Ok(())
}
