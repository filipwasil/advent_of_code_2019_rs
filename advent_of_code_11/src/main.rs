use std::fs;
use std::collections::{HashMap};

struct Orbits{}

impl Orbits
{
    fn execute_instructions(&mut self, filename: &str) -> usize
    {
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file {}");

        let lines = contents.split("\n").filter(|x| !x.is_empty()).map(|x| x);
        let mut m = HashMap::<String, String>::new();
        for line in lines {
           let mut planets = line.split(")");
           let on_orbit = planets.next().unwrap().to_string();
           let owner = planets.next().unwrap().to_string();
           m.entry(owner).or_insert(on_orbit);
        }

        let mut orbits = HashMap::<String, usize>::new();
        let mut orbitsCount: usize = 0;

        for planet in &m {
            let mut p = String::from(planet.0);
            let mut count = 0;
            // print!("Orbits for planet {}: ", planet.0);
            loop
            {
                match m.get(&p)
                {
                    Some(x) => {
                        p = x.to_string();
                        count += 1;
                        // print!("{} ", p);
                    },
                    None => {break;},
                }
            }
//            println!("");
            orbits.entry(planet.0.to_string()).or_insert(count);
        }

        for o in orbits
        {
            orbitsCount += o.1;
        }
        orbitsCount
    }
}

fn main() {
    let mut o = Orbits{};
    let r = o.execute_instructions("input.txt");
    println!("Result: {}", r);
}
