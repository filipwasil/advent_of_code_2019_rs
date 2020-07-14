
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
    relative_base: i64,
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

        let mut memory_program = IntCodeComputer::read_memory(filename);
        let mut memory_data = Vec::with_capacity(1000000);
        memory_data.resize(3000, 0);
        memory_program.append(&mut memory_data);
        IntCodeComputer {
            output: 0
            , last_input: 0
            , pc: 0
            , relative_base: 0
            , memory: memory_program
        }
    }

    pub fn process_instructions(
        &mut self, inputs: Vec<i64>, stop_on_output_write: bool) -> Result<i64, i64> {
        self.process_inputs(inputs, stop_on_output_write)
    }

    fn get_parameters(&mut self, args: (i64, i64), modes: (i64, i64, i64)) -> (i64, i64) {
        (self.get_parameter(args.0, modes.0), self.get_parameter(args.1, modes.1))
    }

    fn get_parameter(&mut self, arg: i64, mode: i64) -> i64 {
        match mode {
            0 => self.memory[arg as usize],
            1 => arg,
            _ => self.memory[(arg + self.relative_base) as usize],
        }
    }

    fn process_inputs(
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
                        let args = self.get_parameters((operant1, operant2), parameter_modes);
                        self.memory[address as usize] = args.0 + args.1;
                        self.pc += 3;
                        println!("PC: {} MODE: {:?} Instruction 1: memory[{}] = {} + {}", self.pc,
                                 parameter_modes,
                                 address, args.0, args.1);
                    },
                    2 =>
                        {
                            let operant1 = self.memory[self.pc];
                            let operant2 = self.memory[self.pc + 1];
                            let address = self.memory[self.pc + 2];
                            let args = self.get_parameters((operant1, operant2), parameter_modes);
                            self.memory[address as usize] = args.0 * args.1;
                            self.pc += 3;
                            println!("PC: {} MODE: {:?} Instruction 2: memory[{}] = {} * {}",
                                     self.pc, parameter_modes, address, args.0, args.1);
                        },
                    3 =>
                        {
                            let next_input = match input_iterator.next() {
                                Some(x) => x,
                                None => {
                                    println!("No more input arguments");
                                    self.last_input
                                },
                            };
                            let param = self.memory[self.pc];
                            let arg = self.get_parameter(param, parameter_modes.0);
//                            let address = self.memory[self.pc];
                            println!("PC: {} MODE: {:?} Instruction 3: arg = {} next input = {} relative base = {}",
                                     self.pc,
                                     parameter_modes,
                                     arg, next_input, self.memory[self.relative_base as usize]);
                            self.memory[arg as usize] = next_input;
                            self.last_input = next_input;
                            assert_eq!(self.memory[arg as usize], next_input);
                            self.pc += 1;
                        },
                    4 =>
                        {
                            let param = self.memory[self.pc];
                            let args = self.get_parameter(param, parameter_modes.0);
                            self.output = args;
                            println!("PC: {} MODE: {:?} Instruction 4: param: {} output = {} relative base = {}", self
                                .pc,
                                     parameter_modes, param, self.output, self.relative_base);
                            self.pc += 1;
                            if stop_on_output_write {
                                return Ok(self.output);
                            }
                        },
                    5 =>
                        {
                            let operant1 = self.memory[self.pc];
                            let operant2 = self.memory[self.pc + 1];
                            let args = self.get_parameters((operant1, operant2), parameter_modes);
                            if args.0 != 0 {
                                self.pc = args.1 as usize;
                            }
                            else {
                                self.pc += 2;
                            }
                        },
                    6 =>
                        {
                            let operant1 = self.memory[self.pc];
                            let operant2 = self.memory[self.pc + 1];
                            let args = self.get_parameters((operant1, operant2), parameter_modes);
                            if args.0 == 0 {
                                self.pc = args.1 as usize;
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
                            let args = self.get_parameters((operant1, operant2), parameter_modes);
                            self.memory[address as usize] = match args.0 < args.1 {
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
                            let args = self.get_parameters((operant1, operant2), parameter_modes);
                            self.memory[address as usize] = match args.0 == args.1 {
                                true => 1,
                                false => 0
                            };
                            self.pc += 3;
                        },
                    9 =>
                        {
                            let param = self.memory[self.pc];
                            let args = self.get_parameter(param, parameter_modes.0);
                            self.relative_base += args;
                            println!("PC: {} MODE: {:?} Instruction 9: relative base = {}", self
                                .pc, parameter_modes, self.relative_base);
                            self.pc += 1;
                        },
                    99 => {
                        println!("PC: {} Halted", self.pc);
                        self.pc = 0;
                        return Err(self.output);
                    },
                    _ => continue
                };
        }
        println!("Program unexpectedly finished");
        Ok(self.output)
    }
}
