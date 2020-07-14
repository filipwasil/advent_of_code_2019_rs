#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use std::cmp::{min, max};
use num_traits::sign::signum;

pub fn get_variants_with_no_repetition(x: i64, y: i64) -> Vec<i64> {
    let mut result = Vec::<i64>::new();
    for n0 in x..y {
        for n1 in x..y {
            if n1 != n0 {
                for n2 in x..y {
                    if n2 != n0 && n2 != n1 {
                        for n3 in x..y {
                            if n3 != n0 && n3 != n1 && n3 != n2{
                                for n4 in x..y {
                                    if n4 != n0 && n4 != n1 && n4 != n2 && n4 != n3 {
                                        result.push(n0 + n1 * 10 + n2 * 100 + n3 * 1000 + n4 * 10000);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    result
}

fn onSegment(p: (i64, i64), q: (i64, i64), r: (i64, i64)) -> bool
{
    q.0 <= max(p.0, r.0) && q.0 >= min(p.0, r.0) && q.1 <= max(p.1, r.1) && q.1 >= min(p.1, r.1)
}

fn orientation(p: (i64, i64), q: (i64, i64), r: (i64, i64) ) -> i64
{
    match signum((q.1 - p.1) * (r.0 - q.0) - (q.0 - p.0) * (r.1 - q.1))
        {
            -1 => 2,
            1 => 1,
            _ => 0,
        }
}

pub fn do_intersect(p1: (i64, i64), q1: (i64, i64), p2: (i64, i64), q2: (i64, i64)) -> bool
{
    let o1 = orientation(p1, q1, p2);
    let o2 = orientation(p1, q1, q2);
    let o3 = orientation(p2, q2, p1);
    let o4 = orientation(p2, q2, q1);

    (o1 != o2 && o3 != o4)
        || (o1 == 0 && onSegment(p1, p2, q1))
        || (o2 == 0 && onSegment(p1, q2, q1))
        || (o3 == 0 && onSegment(p2, p1, q2))
        || (o4 == 0 && onSegment(p2, q1, q2))
}

