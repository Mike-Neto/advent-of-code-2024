use day_03::part2::process;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../data.txt");
    let result = process(file)?;
    println!("{}", result);
    assert_eq!("93729253", result);
    Ok(())
}
