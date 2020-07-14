use std::fs;
use std::cmp::{min, max};
use num_traits::sign::signum;

struct Wires {
}

impl Wires
{
    fn onSegment(&self, p: (i64, i64), q: (i64, i64), r: (i64, i64)) -> bool
    { 
        q.0 <= max(p.0, r.0) && q.0 >= min(p.0, r.0) && q.1 <= max(p.1, r.1) && q.1 >= min(p.1, r.1)
    }

    fn orientation(&self, p: (i64, i64), q: (i64, i64), r: (i64, i64) ) -> i64
    { 
        match signum((q.1 - p.1) * (r.0 - q.0) - (q.0 - p.0) * (r.1 - q.1))
        {
            -1 => 2,
            1 => 1,
            _ => 0,
        }
    }

    pub fn doIntersect(&self, p1: (i64, i64), q1: (i64, i64), p2: (i64, i64), q2: (i64, i64)) -> bool
    { 
        let o1 = self.orientation(p1, q1, p2); 
        let o2 = self.orientation(p1, q1, q2); 
        let o3 = self.orientation(p2, q2, p1); 
        let o4 = self.orientation(p2, q2, q1); 

        (o1 != o2 && o3 != o4)
            || (o1 == 0 && self.onSegment(p1, p2, q1))
            || (o2 == 0 && self.onSegment(p1, q2, q1))
            || (o3 == 0 && self.onSegment(p2, p1, q2))
            || (o4 == 0 && self.onSegment(p2, q1, q2))
    } 

    fn count_intersection(&self, p1: (i64, i64), p2: (i64, i64), p3: (i64, i64), p4: (i64, i64)) -> (i64, i64)
    {
        let pz1 = ((p2.1-p1.1)*(p4.0-p3.0)-(p4.1-p3.1)*(p2.0-p1.0));
        let pz2 = ((p4.1-p3.1)*(p2.0-p1.0)-(p2.1-p1.1)*(p4.0-p3.0));
        if pz1 == 0 || pz2 == 0
        {
            return (1000, 1000);
        }
        let x=((p2.0-p1.0)*(p4.0*p3.1-p4.1*p3.0)-(p4.0-p3.0)*(p2.0*p1.1-p2.1*p1.0))/pz1;
        let y=((p4.1-p3.1)*(p2.0*p1.1-p2.1*p1.0)-(p2.1-p1.1)*(p4.0*p3.1-p4.1*p3.0))/pz2;
        return (x, y)
    }

    fn splitWireIntoSteps(&self, wireData: &str) -> Vec<(i64, i64)>
    {
        let steps = wireData.split(",");

        let mut result = Vec::<(i64, i64)>::with_capacity(30);
        for step in steps {
            match step.chars().nth(0) {
                Some('R') => result.push((step[1..].to_string().parse::<i64>().expect("Could not parse"), 0))
                , Some('D') => result.push((0, -step[1..].to_string().parse::<i64>().expect("Could not parse")))
                , Some('U') => result.push((0, step[1..].to_string().parse::<i64>().expect("Could not parse")))
                , Some('L') => result.push((-step[1..].to_string().parse::<i64>().expect("Could not parse"), 0))
                , Some(_) => {}
                , None => {}
            };
        }
        result
    }

    fn get_nearest_intersection_distance(&self, filename: &str) -> i64
    {
        let mut contents = fs::read_to_string(filename).expect("Something went wrong reading the file {}");

        // Solution
        let mut wires = contents.split("\n");

        let wireOne = wires.next().expect("Wire data not found in input file");
        let wireTwo = wires.next().expect("Wire data not found in input file");
        
        let w1 = self.splitWireIntoSteps(wireOne);
        let w2 = self.splitWireIntoSteps(wireTwo);
        
        let mut nearest: (i64, i64) = (10000, 10000);
        let mut currentpoints: ((i64, i64), (i64, i64)) = ((0,0), (0,0)); // w1, w2

        for w1step in &w1 {
            let w1LineEnd = ((currentpoints.0).0 + w1step.0, (currentpoints.0).1 + w1step.1);
            for w2step in &w2 {
                let w2LineEnd = ((currentpoints.1).0 + w2step.0, (currentpoints.1).1 + w2step.1);
                if self.doIntersect(currentpoints.0, w1LineEnd, currentpoints.1, w2LineEnd) {
                    let r = self.count_intersection(currentpoints.0, w1LineEnd, currentpoints.1, w2LineEnd);
                    if (nearest.0.abs() + nearest.1.abs()) > (r.0.abs() + r.1.abs()) && (r.0.abs() + r.1.abs()) != 0 {
                        nearest = r;
                    }
                }
                currentpoints.1 = w2LineEnd;
            }
            currentpoints.1 = (0, 0);
            currentpoints.0 = w1LineEnd;
        }
        nearest.0 + nearest.1
    } 
}

fn main() {
    let mut computer = Wires{};
    let r = computer.get_nearest_intersection_distance("input.txt");
    println!("Result: {}, {}", r.0, r.1);
}

