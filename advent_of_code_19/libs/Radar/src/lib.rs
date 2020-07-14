#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use std::fs;
use num_traits::sign::signum;

pub struct AsteroidRadar {
    asterids: Vec<usize>
    , size: (usize, usize)
}

impl AsteroidRadar {
    pub fn new() -> AsteroidRadar {
        let map = AsteroidRadar::read_map("input.txt");
        println!("{:?}", map);
        AsteroidRadar {
            asterids: map.0
            , size: (map.1, map.2)
        }
    }

    fn read_map(filename: &str) -> (Vec<usize>, usize, usize) {
        let mut contents = fs::read_to_string(filename).expect("Something went wrong reading the file {}");
        // Workaround - Empty line at the input file end causing collect function to crash
        contents.pop();
        let numbers: Vec<usize> = contents.chars().map(|c| match c { '#' => 1, '.' => 0, _ => 2}).collect();
        let columns = numbers.iter().position(|&r| r == 2).expect("No new line found");
        let data: Vec<usize> = numbers.iter().filter(|x| **x == 1 || **x == 0).map(|x| *x).collect();
        (data, columns, numbers.len() / columns)
    }

    pub fn get_asteroids_count(&self) -> usize {
        self.asterids.iter().filter(|&n| *n == 1).count()
    }

    pub fn get_map_size(&self) -> (usize, usize) {
        self.size
    }

    pub fn get_visible_asteroids_for_point(&self, start: (usize, usize)) -> usize {
        let mut r: usize = 0;
        let mut ba = (0, 0);

        for idx in 0..self.size.0 {
            let val = self.asterids[start.0 + idx * self.size.0];
            if val == 0 {
                continue;
            }
            match signum(start.0 as i64 - idx as i64) {
                1 => match ba.0 {
                        0 => {
                            ba.0 += 1;
                        }
                        , _ => {
                            r += 1;
                        }
                }
                0 => {},
                -1 => match ba.0 {
                    0 => {
                        ba.1 += 1;
                    }
                    , _ => {
                        r += 1;
                    }
                }
                _ => {},
            }
        }

        for idx in 0..self.size.1 {
            let val = self.asterids[idx + start.1 * self.size.0];
            if val == 0 {
                continue;
            }
            match signum(start.1 as i64 - idx as i64) {
                1 => match ba.1 {
                    0 => {
                        ba.1 += 1;
                    }
                    , _ => {
                        r += 1;
                    }
                }
                0 => {},
                -1 => match ba.1 {
                    0 => {
                        ba.1 += 1;
                    }
                    , _ => {
                        r += 1;
                    }
                }
                _ => {},
            }
        }

        // TODO we still have to exclude asteroids with segments that cover one anotheer.
        self.get_asteroids_count() - 1 - r
    }
}

