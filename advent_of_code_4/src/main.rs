
use std::fs;

struct Computer{}

impl Computer
{
    fn execute_instructions(&mut self, filename: &str, noun: usize, verb: usize) -> usize
    {
        let mut contents = fs::read_to_string(filename).expect("Something went wrong reading the file {}");
    
        // Workaroud - Empty line at the input file end causing collect function to crash
        contents.pop();

        // Solution
        let mut numbers = contents.split(",")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<usize>().expect("Could no parse string to u64"));
        
        let mut accesible_numbers: Vec<usize> = contents.split(",")
            .filter(|x| {!x.is_empty()})  
            .map(|x| {x.parse::<usize>().expect("Could no parse string to u64")})
            .collect();  

            accesible_numbers[1] = noun;
            accesible_numbers[2] = verb;
        
        loop 
        {
            let operation = numbers.next();
            let operant1 = match numbers.next() { Some(x) => x, None => 0};
            let operant2 = match numbers.next() { Some(x) => x, None => 0};
            let output = match numbers.next() { Some(x) => x, None => 0};

            let m = accesible_numbers.len();
            if output >= m || operant1 >= m || operant2 >= m
            {
                break; 
            }
            match operation
            {
                Some(1) => accesible_numbers[output] = accesible_numbers[operant1] + accesible_numbers[operant2],
                Some(2) => accesible_numbers[output] = accesible_numbers[operant1] * accesible_numbers[operant2],
                Some(_) => break,
                None => break,
            };
        }

        accesible_numbers[0]
    } 
}

fn find_values() -> (usize, usize)
{
    let targetValue = 19690720;
    let mut computer = Computer{};

    for noun in 0..99
    {
        for verb in 0..99
        {
            if computer.execute_instructions("input.txt", noun, verb) == targetValue
            {
                return (noun, verb);
            }
        }
    }
    return (0, 0);
} 

fn main() {
    let r = find_values();
    println!("Result: {}, {}", r.0, r.1);
}
