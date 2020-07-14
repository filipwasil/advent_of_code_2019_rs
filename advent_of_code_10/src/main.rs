
use std::fs;
use std::convert::TryInto;

struct Computer{}

pub trait ForwardBackwardIterator : Iterator {
    fn prev(&mut self) -> Option<Self::Item>;
}

pub struct VectorForwardBackwardIterator<'a, Item> where Item : 'a {
    index: Option<usize>,
    vector: &'a Vec<Item>,
}

impl<'a, Item> VectorForwardBackwardIterator<'a, Item> {
    fn new(vector: &'a Vec<Item>) -> VectorForwardBackwardIterator<'a, Item> {
        VectorForwardBackwardIterator { index: None, vector: vector }
    }
}

impl<'a, Item> Iterator for VectorForwardBackwardIterator<'a, Item> {
    type Item = &'a Item;

    fn next(&mut self) -> Option<&'a Item> {
        let index = 
            match self.index {
                Some(i) => i + 1,
                None => 0
            };

        self.index = Some(index);
        self.vector.get(index)
    }
}

impl<'a, Item> ForwardBackwardIterator for VectorForwardBackwardIterator<'a, Item> {
    fn prev(&mut self) -> Option<&'a Item> {
        let index = 
            match self.index {
                Some(0) | None => return None,
                Some(i) => i - 1
            };

        self.index = Some(index);
        self.vector.get(index)
    }
}

impl Computer
{
    fn execute_instructions(&mut self, filename: &str, input: i64) -> i64
    {
        let mut contents = fs::read_to_string(filename).expect("Something went wrong reading the file {}");
    
        // Workaroud - Empty line at the input file end causing collect function to crash
        contents.pop();

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
        let mut pc: usize = 0;
        while pc < memory.len()
        {
            let opcode = memory[pc];
            pc += 1;
            let instruction = opcode % 100;
            parameter_modes.0 = opcode/100 %10;
            parameter_modes.1 = opcode/1000 %10;
            parameter_modes.2 = opcode/10000 %10;
//            println!("pc: {} instruction: {} ", pc, opcode);
            match instruction
            {
                1 => {
                    let operant1 = memory[pc];
                    let operant2 = memory[pc + 1];
                    let address = memory[pc + 2];
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
                    pc += 3;
                },
                2 => 
                {
                    let operant1 = memory[pc];
                    let operant2 = memory[pc + 1];
                    let address = memory[pc + 2];
                    let val1 = match parameter_modes.0 {
                        0 => memory[operant1 as usize],
                        _ => operant1
                    };
                    let val2 = match parameter_modes.1 {
                        0 => memory[operant2 as usize],
                        _ => operant2
                    };
                    memory[address as usize] = val1 * val2;
                    pc += 3;
                },
                3 =>
                {
                    let address = memory[pc];
                    memory[address as usize] = input;
                    pc += 1;
                },
                4 =>
                {
                    let address = memory[pc];
                    output = memory[address as usize];
                    pc += 1;
                },
                5 =>
                {
                    let operant1 = memory[pc];
                    let operant2 = memory[pc + 1];
                    let val1 = match parameter_modes.0 {
                        0 => memory[operant1 as usize],
                        _ => operant1
                    };
                    let val2 = match parameter_modes.1 {
                        0 => memory[operant2 as usize],
                        _ => operant2
                    };
                    if val1 != 0 {
                        pc = val2 as usize;
                    }
                    else {
                        pc += 2;
                    }
                },
                6 =>
                {
                    let operant1 = memory[pc];
                    let operant2 = memory[pc + 1];
                    let val1 = match parameter_modes.0 {
                        0 => memory[operant1 as usize],
                        _ => operant1
                    };
                    let val2 = match parameter_modes.1 {
                        0 => memory[operant2 as usize],
                        _ => operant2
                    };
                    if val1 == 0 {
                        pc = val2 as usize;
                    }
                    else {
                        pc += 2;
                    }
                },
                7 =>
                {
                    let operant1 = memory[pc];
                    let operant2 = memory[pc + 1];
                    let address = memory[pc + 2];
                    let val1 = match parameter_modes.0 {
                        0 => memory[operant1 as usize],
                        _ => operant1
                    };
                    let val2 = match parameter_modes.1 {
                        0 => memory[operant2 as usize],
                        _ => operant2
                    };
                    memory[address as usize] = match val1 < val2 {
                        true => 1,
                        false => 0
                    };
                    pc += 3;
                },
                8 =>
                {
                    let operant1 = memory[pc];
                    let operant2 = memory[pc + 1];
                    let address = memory[pc + 2];
                    let val1 = match parameter_modes.0 {
                        0 => memory[operant1 as usize],
                        _ => operant1
                    };
                    let val2 = match parameter_modes.1 {
                        0 => memory[operant2 as usize],
                        _ => operant2
                    };
                    memory[address as usize] = match val1 == val2 {
                        true => 1,
                        false => 0
                    };
                    pc += 3;
                },
                99 => break,
                _ => continue
            };
        }

        output
    } 
}

fn main() {
    let mut computer = Computer{};
    let r = computer.execute_instructions("input.txt", 5);
    println!("Result: {}", r);
}
