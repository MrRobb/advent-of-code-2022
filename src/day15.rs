#![allow(clippy::must_use_candidate, clippy::missing_panics_doc)]

use itertools::Itertools;
use scan_fmt::scan_fmt;

#[derive(Copy, Clone)]
struct Square {
    left: (i64, i64),
    right: (i64, i64),
    top: (i64, i64),
    bottom: (i64, i64),
    beacon: (i64, i64),
}

struct Map {
    sensors: Vec<Square>,
    min: i64,
    max: i64,
}

const fn diamond_to_cartesian((x, y): (i64, i64)) -> (i64, i64) {
    let x_ = x - y;
    let y_ = x + y;
    (x_, y_)
}

const fn cartesian_to_diamond((x_, y_): (i64, i64)) -> (i64, i64) {
    let x = (x_ + y_) / 2;
    let y = y_ - x;
    (x, y)
}

const fn manhattan_distance((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

const fn is_inside((x, y): &(i64, i64), square: &Square) -> bool {
    *x >= square.left.0 && *x <= square.top.0 && *y >= square.left.1 && *y <= square.bottom.1
}

fn build_map(input: &str) -> Map {
    let sensors = input
        .lines()
        .map(|line| {
            let (sensor_x, sensor_y, beacon_x, beacon_y) = scan_fmt!(
                line,
                "Sensor at x={d}, y={d}: closest beacon is at x={d}, y={d}",
                i64,
                i64,
                i64,
                i64
            )
            .unwrap();

            let distance = manhattan_distance((sensor_x, sensor_y), (beacon_x, beacon_y));
            Square {
                left: diamond_to_cartesian((sensor_x - distance, sensor_y)),
                right: diamond_to_cartesian((sensor_x + distance, sensor_y)),
                top: diamond_to_cartesian((sensor_x, sensor_y - distance)),
                bottom: diamond_to_cartesian((sensor_x, sensor_y + distance)),
                beacon: diamond_to_cartesian((beacon_x, beacon_y)),
            }
        })
        .collect::<Vec<_>>();

    let (min, max) = sensors
        .iter()
        .flat_map(|square| {
            [
                cartesian_to_diamond(square.top).0,
                cartesian_to_diamond(square.bottom).0,
                cartesian_to_diamond(square.left).0,
                cartesian_to_diamond(square.right).0,
            ]
        })
        .minmax()
        .into_option()
        .unwrap();

    Map { sensors, min, max }
}

fn no_beacon_at_2m(input: &str) -> usize {
    const ROW: i64 = 2_000_000;
    let map = build_map(input);
    (map.min..map.max)
        .map(|x| diamond_to_cartesian((x, ROW)))
        .filter(|&position| {
            map.sensors
                .iter()
                .any(|sensor| position != sensor.beacon && is_inside(&position, sensor))
        })
        .count()
}

fn find_distress_beacon(input: &str) -> i64 {
    let map = build_map(input);

    let mut distress_signal = (0, 0);

    for sensor in &map.sensors {
        let left = (sensor.left.1..=sensor.bottom.1).map(move |y| (sensor.left.0 - 1, y));
        let top = (sensor.left.0..=sensor.top.0).map(move |x| (x, sensor.top.1 - 1));
        let right = (sensor.top.1..=sensor.right.1).map(move |y| (sensor.right.1 + 1, y));
        let bottom = (sensor.bottom.0..=sensor.right.0).map(move |x| (x, sensor.bottom.0 + 1));

        // Iter over all the points in the square
        let possible_distress_signal = left
            .chain(right)
            .chain(top)
            .chain(bottom)
            .filter(|&point| {
                let position = cartesian_to_diamond(point);
                position.0 >= 0 && position.0 <= 4_000_000 && position.1 >= 0 && position.1 <= 4_000_000
            })
            .find(|point| map.sensors.iter().all(|s| point != &s.beacon && !is_inside(point, s)));
        if let Some(point) = possible_distress_signal {
            distress_signal = cartesian_to_diamond(point);
        }
    }

    distress_signal.0 * 4_000_000 + distress_signal.1
}

pub fn main() {
    let input = std::fs::read_to_string("input/day15.txt").expect("Input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", no_beacon_at_2m(&input));
    println!("PART 2 = {}", find_distress_beacon(&input));
    println!("Execution time: {:?}", now.elapsed());
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_cart_diamond_conversion(x in -1000_i64..1000, y in -1000_i64..1000) {
            let cart = diamond_to_cartesian((x, y));
            let diam = cartesian_to_diamond(cart);
            prop_assert_eq!(diam, (x, y));
        }
    }
}
