use day_04::part1::process;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../data.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}
