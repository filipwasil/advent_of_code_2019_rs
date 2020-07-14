use Computer::IntCodeComputer;
use std::cmp::max;

use Utils::get_variants_with_no_repetition;

fn main() {
    let phases = get_variants_with_no_repetition(0, 5);
    let mut r = 0;
    for phase in phases {
        let mut last_output = 0;
        let mut divider_phase = 10000;
        println!("Phase: {}", phase);
        while divider_phase > 0 {
            let mut computer = IntCodeComputer::new();
            let phase_setting = phase / divider_phase % 10;
            last_output = computer.process_instructions("input.txt", vec!(phase_setting, last_output));

            divider_phase /= 10;
        }
        r = max(last_output, r);
    }
    println!("Result: {}", r);
}
