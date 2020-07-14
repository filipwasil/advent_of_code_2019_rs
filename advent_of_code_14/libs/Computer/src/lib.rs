
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
    last_input: i64,
    pc: usize,
    memory: Vec<i64>
}

impl IntCodeComputer
{

    pub fn read_memory(filename: &str) -> Vec<i64>
    {
        let mut contents = fs::read_to_string(filename).expect("Something went wrong reading the file {}");
        // Workaround - Empty line at the input file end causing collect function to crash
        contents.pop();

        contents.split(",")
            .filter(|x| {!x.is_empty()})
            .map(|x| {x.parse::<i64>().expect("Could no parse string to u64")})
            .collect()
    }

    pub fn new(filename: &str) -> IntCodeComputer
    {
        let mut contents = fs::read_to_string(filename).expect("Something went wrong reading the file {}");
        // Workaround - Empty line at the input file end causing collect function to crash
        contents.pop();

        IntCodeComputer {
            output: 0
            , last_input: 0
            , pc: 0
            , memory: IntCodeComputer::read_memory(filename)
        }
    }

    pub fn process_instructions(&mut self, inputs: Vec<i64>, stop_on_output_write: bool) ->
                                                                                        Result<i64, i64>
    {
        self.process_inputs(inputs, stop_on_output_write)
    }

    pub fn process_inputs(
        &mut self
        , input: Vec<i64>
        , stop_on_output_write: bool) -> Result<i64, i64>
    {
        let mut input_iterator = input.into_iter();

        while self.pc < self.memory.len() {
            let opcode = self.memory[self.pc];
            self.pc += 1;
            let instruction = opcode % 100;
            let parameter_modes = (opcode/100 %10, opcode/1000 %10, opcode/10000 %10);
            match instruction
                {
                    1 => {
                        let operant1 = self.memory[self.pc];
                        let operant2 = self.memory[self.pc + 1];
                        let address = self.memory[self.pc + 2];
                        let val1 = match parameter_modes.0 {
                            0 => self.memory[operant1 as usize],
                            _ => operant1
                        };
                        let val2 = match parameter_modes.1 {
                            0 => self.memory[operant2 as usize],
                            _ => operant2
                        };
                        self.memory[address as usize] = val1 + val2;
                        self.pc += 3;
                        println!("PC: {} Instruction 1: memory[{}] = {} + {}", self.pc, address, val1, val2);
                    },
                    2 =>
                        {
                            let operant1 = self.memory[self.pc];
                            let operant2 = self.memory[self.pc + 1];
                            let address = self.memory[self.pc + 2];
                            let val1 = match parameter_modes.0 {
                                0 => self.memory[operant1 as usize],
                                _ => operant1
                            };
                            let val2 = match parameter_modes.1 {
                                0 => self.memory[operant2 as usize],
                                _ => operant2
                            };
                            self.memory[address as usize] = val1 * val2;
                            self.pc += 3;
                            println!("PC: {} Instruction 2: memory[{}] = {} * {}", self.pc, address, val1, val2);
                        },
                    3 =>
                        {
                            let address = self.memory[self.pc];
                            let next_input = match input_iterator.next() {
                                Some(x) => x,
                                None => {
                                    println!("No more input arguments");
                                    self.last_input
                                },
                            };
                            println!("PC: {} Instruction 3: memory[{}] = {}", self.pc, address, next_input);
                            self.memory[address as usize] = next_input;
                            self.last_input = next_input;
                            assert_eq!(self.memory[address as usize], next_input);
                            self.pc += 1;
                        },
                    4 =>
                        {
                            let address = self.memory[self.pc];
                            self.output = self.memory[address as usize];
                            println!("PC: {} Instruction 4: memory[{}] {}", self.pc, address, self.memory[address as usize]);
                            self.pc += 1;
                            if stop_on_output_write {
                                return Ok(self.output);
                            }
                        },
                    5 =>
                        {
                            let operant1 = self.memory[self.pc];
                            let operant2 = self.memory[self.pc + 1];
                            let val1 = match parameter_modes.0 {
                                0 => self.memory[operant1 as usize],
                                _ => operant1
                            };
                            let val2 = match parameter_modes.1 {
                                0 => self.memory[operant2 as usize],
                                _ => operant2
                            };
                            if val1 != 0 {
                                self.pc = val2 as usize;
                            }
                            else {
                                self.pc += 2;
                            }
                        },
                    6 =>
                        {
                            let operant1 = self.memory[self.pc];
                            let operant2 = self.memory[self.pc + 1];
                            let val1 = match parameter_modes.0 {
                                0 => self.memory[operant1 as usize],
                                _ => operant1
                            };
                            let val2 = match parameter_modes.1 {
                                0 => self.memory[operant2 as usize],
                                _ => operant2
                            };
                            if val1 == 0 {
                                self.pc = val2 as usize;
                            }
                            else {
                                self.pc += 2;
                            }
                        },
                    7 =>
                        {
                            let operant1 = self.memory[self.pc];
                            let operant2 = self.memory[self.pc + 1];
                            let address = self.memory[self.pc + 2];
                            let val1 = match parameter_modes.0 {
                                0 => self.memory[operant1 as usize],
                                _ => operant1
                            };
                            let val2 = match parameter_modes.1 {
                                0 => self.memory[operant2 as usize],
                                _ => operant2
                            };
                            self.memory[address as usize] = match val1 < val2 {
                                true => 1,
                                false => 0
                            };
                            self.pc += 3;
                        },
                    8 =>
                        {
                            let operant1 = self.memory[self.pc];
                            let operant2 = self.memory[self.pc + 1];
                            let address = self.memory[self.pc + 2];
                            let val1 = match parameter_modes.0 {
                                0 => self.memory[operant1 as usize],
                                _ => operant1
                            };
                            let val2 = match parameter_modes.1 {
                                0 => self.memory[operant2 as usize],
                                _ => operant2
                            };
                            self.memory[address as usize] = match val1 == val2 {
                                true => 1,
                                false => 0
                            };
                            self.pc += 3;
                        },
                    99 => {
                        println!("PC: {} Halted", self.pc);
                        return Err(self.output);
                    },
                    _ => continue
                };
        }
        Ok(self.output)
    }
}
