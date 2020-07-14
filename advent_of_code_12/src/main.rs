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

        let mut orbits = HashMap::<String, Vec<String> >::new();
        let mut stepsHome: usize = 0;

        for planet in &m {
            let mut p = String::from(planet.0);
            let mut found = (true, false);
            let mut road_home = Vec::<String>::new();
            if planet.0 != "SAN" && planet.0 != "YOU" {
                continue;
            }
            
            print!("Orbits for planet {}: ", planet.0);
            
            loop
            {
                match m.get(&p)
                {
                    Some(x) => {
                        p = x.to_string();
                        road_home.push(x.to_string());
                        if found.0 {
                            print!("{} ", p);
                        }
                    },
                    None => {break;},
                }
            }
            if found.0 {
                println!("\n\n");
            }
            orbits.entry(planet.0.to_string()).or_insert(road_home);
        }

        let santa_orbits = orbits.get("SAN").expect("").into_iter();
        
        let mut common = String::from("");
        let mut distance_to = (0, 0);
        'inner: for os in santa_orbits
        {
            distance_to.0 += 1;
            let my_orbits = orbits.get("YOU").expect("").into_iter();
            for om in my_orbits
            {
                distance_to.1 += 1;
                //println!("Compare om: {} os: {}", om, os);
                if os == om
                {
                    println!("Same");
                    common = om.to_string();
                    break 'inner;
                }
            }
            distance_to.1 = 0;
        }
        println!("Common element: {}", common);
        distance_to.0 + distance_to.1 - 2
    }
}

fn main() {
    let mut o = Orbits{};
    let r = o.execute_instructions("input.txt");
    println!("Result: {}", r);
}
