use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let fuel = contents.split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u64>().expect("Could no parse string to u32"))
        .fold(0, |sum: u64, new: u64| {
            sum + (new / 3) - 2
        });

    print!("Result: {}", fuel);
}
