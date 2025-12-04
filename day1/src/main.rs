use day1::*;
use std::fs::File;
use std::fs::read_to_string;

fn main() {
    part1();
}

fn part1() {
    // Part 1
    let mut dial = Dial::new(50).unwrap();
    let mut total_zeros_count = 0;

    let test_input = read_to_string("input/input.txt").unwrap();

    let dial_rotation_strings = test_input.split('\n').collect::<Vec<&str>>();

    let mut dial_rotations:Vec<DialRotation> = vec![];

    for s in dial_rotation_strings {
        let dial_rotation = get_dial_rotation(s);
        dial_rotations.push(dial_rotation);
    }

    for r in dial_rotations {
        dial.turn_dial_in_direction(r);
        if dial.get_current_position() == 0 {
            total_zeros_count += 1;
        }
    }

    println!("(Part 1): Total zeros count: {}", total_zeros_count);
}