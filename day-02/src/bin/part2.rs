use day_02::part2::process;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../data.txt");
    let result = process(file)?;
    println!("there are {} safe reports", result);
    assert_eq!("598", result);
    Ok(())
}
