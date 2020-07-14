use std::fs;

struct FuelCalculator<T> {  
    result: T,
}

impl FuelCalculator<i64>
{

    fn get_fuel_needed(&self, mass: i64) -> i64
    {
        (mass / 3) - 2
    }

    fn get_fuel_recursive(&self, filename: &str) -> i64
    {
        println!("Filename: {}", filename);
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file {}");
    
        contents.split("\n")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i64>().expect("Could no parse string to u32"))
            .fold(0, |sum: i64, new: i64| {
                println!("new: {}", new);
                let mut localSum = 0;
                let mut result = self.get_fuel_needed(new);
                while result > 0 
                {
                    localSum += result;
                    result = self.get_fuel_needed(result);
                }
                println!("Value: {}", sum + localSum);
                sum + localSum
            })
    } 
}

fn main() {
    println!("Result: {}", FuelCalculator::<i64>{result: 0}.get_fuel_recursive(".\\input.txt"));
}
