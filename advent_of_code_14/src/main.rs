use Computer::IntCodeComputer;
use std::cmp::max;

use Utils::get_variants_with_no_repetition;

fn main() {
    let phases = get_variants_with_no_repetition(5, 10);
    let mut r = 0;
    for phase in phases {
        println!("Phase: {}", phase);
        let mut amplifiers =
            [IntCodeComputer::new("input.txt")
                , IntCodeComputer::new("input.txt")
                , IntCodeComputer::new("input.txt")
                , IntCodeComputer::new("input.txt")
                , IntCodeComputer::new("input.txt")];
        let phases_idx= [10000, 1000, 100, 10, 1];

        // init phase
        let mut last_output = 0;
        for amp_idx in 5..10 {
            println!("Amplifier: {} initialization", amp_idx - 5);
            let phase_setting = (phase / (phases_idx[amp_idx - 5])) % 10;
            last_output = match amplifiers[amp_idx - 5 as usize]
                .process_instructions(vec!(phase_setting, last_output), true) {
                Ok(x) => x,
                Err(x) => x
            };
        }

        'outer: loop {
            for amp_idx in 0..5 {
                println!("Amplifier: {} Input: {}", amp_idx, last_output);
                match amplifiers[amp_idx as usize].process_instructions(vec!(last_output), true) {
                    Ok(x) => {
                        last_output = x;
                    },
                    Err(x) => {
                        last_output = x;
                        if amp_idx == 4 {
                            break 'outer;
                        }
                    }
                }
            }
        }

        r = max(last_output, r);
    }
    println!("Result: {}", r);
}
