use std::fs;

struct Computer<T> {  
    pub result: T,
}

impl Computer<usize>
{
    fn execute_instructions(&mut self, filename: &str)
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

            accesible_numbers[1] = 12;
            accesible_numbers[2] = 2;
        
        loop 
        {
            let operation = numbers.next();
            let operant1 = match numbers.next() { Some(x) => x, None => 0};
            let operant2 = match numbers.next() { Some(x) => x, None => 0};
            let output = match numbers.next() { Some(x) => x, None => 0};

            match operation
            {
                Some(1) => accesible_numbers[output] = accesible_numbers[operant1] + accesible_numbers[operant2],
                Some(2) => accesible_numbers[output] = accesible_numbers[operant1] * accesible_numbers[operant2],
                Some(_) => break,
                None => break,
            };
        }

        self.result = accesible_numbers[0]
    } 
}

fn main() {
    let mut computer = Computer {result: 0};
    computer.execute_instructions("input.txt");
    println!("Result: {}", computer.result);
}
