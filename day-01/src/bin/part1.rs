use day_01::part1::process;

fn main() {
    let file = include_str!("../../data.txt");
    let result = process(file).unwrap();
    println!("The total distance between your lists is: {result}");
    assert_eq!(result, String::from("3508942"));
}
