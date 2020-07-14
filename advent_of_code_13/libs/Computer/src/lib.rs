#[macro_use] extern crate log;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use std::fs;

pub struct IntCodeComputer{
    output: i64,
}

impl IntCodeComputer
{
    pub fn new() -> IntCodeComputer
    {
        IntCodeComputer {output: 0}
    }

    pub fn process_instructions(&mut self, filename: &str, inputs: Vec<i64>) -> i64
    {
        let mut contents = fs::read_to_string(filename).expect("Something went wrong reading the file {}");
        // Workaround - Empty line at the input file end causing collect function to crash
        contents.pop();

        let mut memory: Vec<i64> = contents.split(",")
            .filter(|x| {!x.is_empty()})
            .map(|x| {x.parse::<i64>().expect("Could no parse string to u64")})
            .collect();

        self.process_inputs(&mut memory, inputs)
    }

    pub fn process_inputs(&mut self, memory: &mut Vec<i64>, input: Vec<i64>) -> i64
    {
        let mut parameter_modes = (0, 0, 0);
        let mut pc: usize = 0;
        let mut input_iterator = input.into_iter();
        while pc < memory.len()
            {
                let opcode = memory[pc];
                pc += 1;
                let instruction = opcode % 100;
                parameter_modes.0 = opcode/100 %10;
                parameter_modes.1 = opcode/1000 %10;
                parameter_modes.2 = opcode/10000 %10;
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
                            memory[address as usize] = val1 + val2;
                            pc += 3;
                            println!("Instruction 1: memory[{}] = {} + {}", address, val1, val2);
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
                                println!("Instruction 2: memory[{}] = {} * {}", address, val1, val2);
                            },
                        3 =>
                            {
                                let address = memory[pc];
                                let next_input = input_iterator.next().expect("Could not get next input");
                                println!("Instruction 3: memory[{}] = {}", address, next_input);
                                memory[address as usize] = next_input;
                                assert_eq!(memory[address as usize], next_input);
                                pc += 1;
                            },
                        4 =>
                            {
                                let address = memory[pc];
                                self.output = memory[address as usize];
                                println!("Instruction 4: memory[{}] {}", address, memory[address as usize]);
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
        self.output
    }
}
