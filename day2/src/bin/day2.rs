use std::path::Path;
use day2::split_input_into_id_ranges;

fn main() {
    // part1();
    part2();
}

fn part1() {
    let split_ranges = split_input_into_id_ranges(Path::new("input/input.txt").into());

    let mut invalid_ids:Vec<u64> = Vec::new();

    for id_range in split_ranges {
        for val in id_range.min..=id_range.max {
            if id_range.is_invalid_id_part1(val) {
                invalid_ids.push(val);
            }
        }
    }

    let sum_of_invalid_ids:u64 = invalid_ids.iter().sum::<u64>();

    println!("Sum of invalid ids: {}", sum_of_invalid_ids);
}

fn part2() {
    let split_ranges = split_input_into_id_ranges(Path::new("input/input.txt").into());

    let mut invalid_ids:Vec<u64> = Vec::new();

    for id_range in split_ranges {
        for val in id_range.min..=id_range.max {
            if id_range.is_invalid_id_part2(val) {
                invalid_ids.push(val);
            }
        }
    }

    let sum_of_invalid_ids:u64 = invalid_ids.iter().sum::<u64>();

    println!("Sum of invalid ids: {}", sum_of_invalid_ids);
}