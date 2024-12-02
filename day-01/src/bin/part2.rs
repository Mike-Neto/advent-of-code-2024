use day_01::part2::process;

fn main() {
    let file = include_str!("../../data.txt");
    let result = process(file).unwrap();
    println!("their similarity score is: {result}");
}
