use Computer::IntCodeComputer;

fn main() {
    let mut computer = IntCodeComputer::new("input.txt");
    let mut r = 0;
    while r == 0 {
        r = match computer.process_instructions(vec!(1), false) {
            Ok(x) => x,
            Err(x) => x,
        };
    }

    println!("Result: {}", r);
}
