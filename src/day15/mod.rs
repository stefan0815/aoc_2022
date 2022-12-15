use std::{collections::HashSet, fs};

#[derive(Clone)]
struct Sensor {
    pos: (i32, i32),
    beacon: (i32, i32),
    distance: i32,
}

fn distance(pos1: (i32, i32), pos2: (i32, i32)) -> i32 {
    return (pos1.0 - pos2.0).abs() + (pos1.1 - pos2.1).abs();
}

fn solve_part_one(sensors: &Vec<Sensor>) -> usize {
    let mut hashset: HashSet<(i32, i32)> = HashSet::new();
    let y_part_one = 2000000;
    for sensor in sensors {
        // println!("{}", sensor.distance);
        let diff = (sensor.pos.1 - y_part_one).abs();
        for x in
            sensor.pos.0 - (sensor.distance - diff)..sensor.pos.0 + (sensor.distance - diff) + 1
        {
            hashset.insert((x, y_part_one));
        }
    }
    for sensor in sensors {
        hashset.remove(&sensor.beacon);
    }

    return hashset.len();
}

fn solve_part_two(sensors: &Vec<Sensor>) -> (i32, i32) {
    let border = 4000000;
    let mut x = 0;
    while x < border {
        // if x % 100000 == 0 {
        //     println!("Scanned x: {}", x);
        // }
        let mut y = 0;
        while y < border {
            let mut found = true;
            for sensor in sensors {
                let distance = distance(sensor.pos, (x, y));
                if distance <= sensor.distance {
                    y = (sensor.distance + sensor.pos.1) - (sensor.pos.0 - x).abs();
                    found = false;
                    break;
                }
            }
            if found {
                return (x, y);
            }
            y += 1;
        }
        x += 1;
    }
    return (0, 0);
}

pub fn solver() {
    let input = fs::read_to_string("./src/day15/input.txt")
        .expect("Should have been able to read the file");
    let sensors_and_beacons: Vec<&str> = input.split("\r\n").collect();

    let mut sensors: Vec<Sensor> = Vec::new();
    for sensor_and_beacon in sensors_and_beacons {
        let split: Vec<&str> = sensor_and_beacon[12..]
            .split(": closest beacon is at x=")
            .collect();

        let split_sensor: Vec<&str> = split[0].split(", y=").collect();
        let split_beacon: Vec<&str> = split[1].split(", y=").collect();
        let sensor_pos = (
            split_sensor[0].parse::<i32>().unwrap(),
            split_sensor[1].parse::<i32>().unwrap(),
        );
        let beacon_pos = (
            split_beacon[0].parse::<i32>().unwrap(),
            split_beacon[1].parse::<i32>().unwrap(),
        );
        sensors.push(Sensor {
            pos: sensor_pos,
            beacon: beacon_pos,
            distance: distance(sensor_pos, beacon_pos),
        });
    }

    println!("Day15:");
    let sum_part_one = solve_part_one(&sensors);
    println!("Number of empty positions in row: {}", sum_part_one);

    let distress = solve_part_two(&sensors);
    println!(
        "Distress signal is at: {}, {}, frequency is {}",
        distress.0,
        distress.1,
        distress.0 as i128 * 4000000 as i128 + distress.1 as i128
    );
}
