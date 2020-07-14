use Radar;
use std::cmp::max;

fn main() {
    let r = Radar::AsteroidRadar::new();
    let size = r.get_map_size();
    let mut result = (0, 0, 0);
    for x in 0..size.0 {
        for y in 0..size.1 {
            let astertoids_count =r.get_visible_asteroids_for_point((x,y));
            if result.0 < astertoids_count {
                result = (astertoids_count, x, y);
            }
        }
    }
    println!("Result: {} Idx {} {}", result.0, result.1, result.2);
}
