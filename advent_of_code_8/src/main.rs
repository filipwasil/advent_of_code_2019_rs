
struct SmartGuy {
}

impl SmartGuy
{
    fn get_passwords(&self, start: i64, end: i64) -> i64
    { 
        let check_increase = |num: &(i64, i64, i64, i64, i64, i64)| {
            num.0 <= num.1 && num.1 <= num.2 && num.2 <= num.3 && num.3 <= num.4 && num.4 <= num.5
        };

        let check_adjacent = |num: &(i64, i64, i64, i64, i64, i64)| {
            let c = [true, num.1 > num.0, num.2 > num.1, num.3 > num.2, num.4 > num.3, num.5 > num.4, true];
            for i in 0..c.len()-2
            {
                if c[i] && !c[i+1] && c[i+2]
                {
                    return true;
                }
            }
            false
        };

        let mut count = 0;
        for val in start..end
        {
            let r = (val/100000 % 10, val/10000 % 10, val/1000 % 10, val/100 % 10, val/10 % 10, val % 10);    
            if !check_increase(&r) {
                continue;
            }
            if check_adjacent(&r)
            {
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
