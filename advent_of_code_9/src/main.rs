
use std::fs;
use std::convert::TryInto;

struct Computer{}

impl Computer
{
    fn execute_instructions(&mut self, filename: &str, input: i64) -> i64
    {
        let mut contents = fs::read_to_string(filename).expect("Something went wrong reading the file {}");
    
        // Workaroud - Empty line at the input file end causing collect function to crash
        //contents.pop();

        // Solution
        let mut numbers = contents.split(",")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i64>().expect("Could no parse string to u64"));
        
        let mut memory: Vec<i64> = contents.split(",")
            .filter(|x| {!x.is_empty()})  
            .map(|x| {x.parse::<i64>().expect("Could no parse string to u64")})
            .collect();
        
        let mut output = 0;

        let mut parameter_modes = (0, 0, 0);
        loop 
        {
            let opcode =  numbers.next().expect("Could no parse string to u64");
            let instruction = opcode % 100;
            parameter_modes.0 = opcode/100 %10;
            parameter_modes.1 =  opcode/1000 %10;
            parameter_modes.2 = opcode/10000 %10;
            println!("instruction: {}", opcode);
            match instruction
            {
                0 => continue,
                1 => {
                    let operant1 = match numbers.next() { Some(x) => x, None => 0};
                    let operant2 = match numbers.next() { Some(x) => x, None => 0};
                    let address = match numbers.next() { Some(x) => x, None => 0};
                    let val1 = match parameter_modes.0 {
                        0 => memory[operant1 as usize],
                        _ => operant1
                    };
                    let val2 = match parameter_modes.1 {
                        0 => memory[operant2 as usize],
                        _ => operant2
                    };
                    println!("instruction 1: {} {} {}", val1, val2, address);
                    memory[address as usize] = val1 + val2;
                },
                2 => 
                {
                    let operant1 = match numbers.next() { Some(x) => x, None => 0};
                    let operant2 = match numbers.next() { Some(x) => x, None => 0};
                    let address = match numbers.next() { Some(x) => x, None => 0};
                    let val1 = match parameter_modes.0 {
                        0 => memory[operant1 as usize],
                        _ => operant1
                    };
                    let val2 = match parameter_modes.1 {
                        0 => memory[operant2 as usize],
                        _ => operant2
                    };
                    memory[address as usize] = val1 * val2;
                },
                3 =>
                {
                    let address = match numbers.next() { Some(x) => x, None => 0};
                    memory[address as usize] = input;
                },
                4 =>
                {
                    let address = match numbers.next() { Some(x) => x, None => 0};
                    output = memory[address as usize];
                },
                99 => break,
                _ => break
            };
        }

        output
    } 
}

fn main() {
    let mut computer = Computer{};
    let r = computer.execute_instructions("input.txt", 1);
    println!("Result: {}", r);
}
