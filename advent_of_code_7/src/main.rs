use std::cmp::{min, max};

struct SmartGuy {
}

impl SmartGuy
{
    fn get_passwords(&self, start: i64, end: i64) -> i64
    { 
        let mut count = 0;

        let check_increase = |num: &(i64, i64, i64, i64, i64, i64)| num.0 <= num.1 && num.1 <= num.2 && num.2 <= num.3 && num.3 <= num.4 && num.4 <= num.5;
        let check_adjacent = |num: &(i64, i64, i64, i64, i64, i64)| num.0 == num.1 || num.1 == num.2 || num.2 == num.3 || num.3 == num.4 || num.4 == num.5;

        for val in start..end
        {
            let r = (val/100000 % 10, val/10000 % 10, val/1000 % 10, val/100 % 10, val/10 % 10, val % 10);
            
            if check_adjacent(&r) && check_increase(&r)
            {
                //println!("E: {} {} {} {} {} {}", r.0, r.1, r.2, r.3, r.4, r.5);
                count += 1;
            }
        }
        count
    }
}

fn main() {
    let r = SmartGuy{}.get_passwords(246515, 739105);
    println!("Result: {}", r);
}

