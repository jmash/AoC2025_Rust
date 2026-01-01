use std::fs::{read_to_string, File};
use std::path::Path;

#[derive(PartialEq, Debug)]
pub struct IdRange {
    pub min: u64,
    pub max: u64,
}

impl IdRange {
    fn new(min: u64, max: u64) -> IdRange {
        IdRange { min, max }
    }

    pub fn is_invalid_id_part1(&self, val:u64) -> bool {
        let val_string = val.to_string();

        let first_half = &val_string[0..val_string.len()/2];
        let second_half = &val_string[val_string.len()/2..];

        first_half == second_half
    }

    pub fn is_invalid_id_part2(&self, val:u64) -> bool {
        let val_string = val.to_string();
        let mut is_valid = false;

        for n in 0..val_string.len() / 2 {
            let current_sub_string = &val_string[0..n+1];
            let current_val_string_chunks:Vec<String> = val_string
                .chars()
                .collect::<Vec<char>>()
                .chunks(n+1)
                .map(|chunk| chunk.iter()
                    .collect::<String>())
                .collect();

            is_valid = current_val_string_chunks.iter().all(|sub_string| {
                sub_string == current_sub_string
            });
            if is_valid == true {
                return true;
            }
        }
        false
    }
}

pub fn split_input_into_id_ranges(file_path: Box<Path>) -> Vec<IdRange> {
    let test_input_file_contents = read_to_string(file_path).unwrap();
    test_input_file_contents
        .split(",")
        .map(|pair| {
            let (min, max) = pair.split_once('-').unwrap();
            IdRange::new(min.parse::<u64>().unwrap(), max.parse::<u64>().unwrap())
        })
        .collect()
}

#[cfg(test)]
mod setup_tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_input_file_loads_successfully() {
        let test_input_file = File::open("input/test_input.txt");
        assert!(test_input_file.is_ok());
    }

    #[test]
    fn test_input_contains_expected_content() {
        let test_input_file_contents = read_to_string("input/test_input.txt").unwrap();

        assert!(test_input_file_contents.contains("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"));
    }

    #[test]
    fn splitting_test_input_gives_expected_string_vector() {
        let test_input_file_contents = read_to_string("input/test_input.txt").unwrap();

        let comparison_vector = vec!["11-22", "95-115", "998-1012", "1188511880-1188511890", "222220-222224", "1698522-1698528", "446443-446449", "38593856-38593862", "565653-565659", "824824821-824824827", "2121212118-2121212124"];

        let test_input_split = test_input_file_contents.split(",").collect::<Vec<&str>>();

        assert!(test_input_split.iter().all(|s| comparison_vector.contains(s)));
    }

    #[test]
    fn splitting_test_input_into_ranges_gives_expected_result() {
        let comparison_range_vector = vec![IdRange::new(11, 22), IdRange::new(95, 115), IdRange::new(998, 1012), IdRange::new(1188511880, 1188511890), IdRange::new(222220, 222224), IdRange::new(1698522, 1698528), IdRange::new(446443, 446449), IdRange::new(38593856, 38593862), IdRange::new(565653, 565659), IdRange::new(824824821, 824824827), IdRange::new(2121212118, 2121212124)];

        let split_ranges = split_input_into_id_ranges(Path::new("input/test_input.txt").into());

        assert!(split_ranges.iter().all(|t| comparison_range_vector.contains(t)));
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn id_of_11_is_invalid() {
        let id_range = IdRange::new(11, 22);
        assert!(id_range.is_invalid_id_part1(11));
    }

    #[test]
    fn id_of_22_is_invalid() {
        let id_range = IdRange::new(11, 22);
        assert!(id_range.is_invalid_id_part1(22));
    }

    #[test]
    fn id_of_95_is_valid() {
        let id_range = IdRange::new(95, 115);
        assert!(!id_range.is_invalid_id_part1(95));
    }

    #[test]
    fn id_of_1010_is_invalid() {
        let id_range = IdRange::new(998, 1012);
        assert!(id_range.is_invalid_id_part1(1010));
    }

    #[test]
    fn id_of_1188511885_is_invalid() {
        let id_range = IdRange::new(1188511880, 1188511890);
        assert!(id_range.is_invalid_id_part1(1188511885));
    }

    #[test]
    fn id_of_222222_is_invalid() {
        let id_range = IdRange::new(222220, 222224);
        assert!(id_range.is_invalid_id_part1(222222));
    }

    #[test]
    fn id_of_446446_is_invalid() {
        let id_range = IdRange::new(446443, 446449);
        assert!(id_range.is_invalid_id_part1(446446));
    }

    #[test]
    fn id_of_38593859_is_invalid() {
        let id_range = IdRange::new(38593856, 38593862);
        assert!(id_range.is_invalid_id_part1(38593859));
    }

    #[test]
    fn range_of_11_to_22_has_two_invalid_ids() {
        let range = IdRange::new(11, 22);

        let mut invalid_count = 0;

        for val in range.min..=range.max {
            if range.is_invalid_id_part1(val) {
                invalid_count += 1;
            }
        }

        assert_eq!(invalid_count, 2);
    }

    #[test]
    fn range_of_998_to_1012_has_one_invalid_id() {
        let range = IdRange::new(998, 1012);

        let mut invalid_count = 0;

        for val in range.min..=range.max {
            if range.is_invalid_id_part1(val) {
                invalid_count += 1;
            }
        }

        assert_eq!(invalid_count, 1);
    }

    #[test]
    fn range_of_1188511880_to_1188511890_has_one_invalid_id() {
        let range = IdRange::new(1188511880, 1188511890);

        let mut invalid_count = 0;

        for val in range.min..=range.max {
            if range.is_invalid_id_part1(val) {
                invalid_count += 1;
            }
        }

        assert_eq!(invalid_count, 1);
    }

    #[test]
    fn range_of_222220_to_222224_has_one_invalid_id() {
        let range = IdRange::new(222220, 222224);

        let mut invalid_count = 0;

        for val in range.min..=range.max {
            if range.is_invalid_id_part1(val) {
                invalid_count += 1;
            }
        }

        assert_eq!(invalid_count, 1);
    }

    #[test]
    fn range_of_1698522_to_1698528_has_no_invalid_ids() {
        let range = IdRange::new(1698522, 1698528);

        let mut invalid_count = 0;

        for val in range.min..=range.max {
            if range.is_invalid_id_part1(val) {
                invalid_count += 1;
            }
        }

        assert_eq!(invalid_count, 0);
    }

    #[test]
    fn range_of_446443_to_446449_has_one_invalid_id() {
        let range = IdRange::new(446443, 446449);

        let mut invalid_count = 0;

        for val in range.min..=range.max {
            if range.is_invalid_id_part1(val) {
                invalid_count += 1;
            }
        }

        assert_eq!(invalid_count, 1);
    }

    #[test]
    fn range_of_38593856_to_38593862_has_one_invalid_id() {
        let range = IdRange::new(38593856, 38593862);

        let mut invalid_count = 0;

        for val in range.min..=range.max {
            if range.is_invalid_id_part1(val) {
                invalid_count += 1;
            }
        }

        assert_eq!(invalid_count, 1);
    }

    #[test]
    fn range_of_565653_to_565659_has_no_invalid_ids() {
        let range = IdRange::new(565653, 565659);

        let mut invalid_count = 0;

        for val in range.min..=range.max {
            if range.is_invalid_id_part1(val) {
                invalid_count += 1;
            }
        }

        assert_eq!(invalid_count, 0);
    }

    #[test]
    fn range_of_824824821_to_824824827_has_no_invalid_ids() {
        let range = IdRange::new(824824821, 824824827);

        let mut invalid_count = 0;

        for val in range.min..=range.max {
            if range.is_invalid_id_part1(val) {
                invalid_count += 1;
            }
        }

        assert_eq!(invalid_count, 0);
    }

    #[test]
    fn range_of_2121212118_to_2121212124_has_no_invalid_ids() {
        let range = IdRange::new(824824821, 824824827);

        let mut invalid_count = 0;

        for val in range.min..=range.max {
            if range.is_invalid_id_part1(val) {
                invalid_count += 1;
            }
        }

        assert_eq!(invalid_count, 0);
    }

    #[test]
    fn sum_of_invalid_ids_in_test_input_is_1227775554() {
        let split_ranges = split_input_into_id_ranges(Path::new("input/test_input.txt").into());

        let mut invalid_ids:Vec<u64> = Vec::new();

        for id_range in split_ranges {
            for val in id_range.min..=id_range.max {
                if id_range.is_invalid_id_part1(val) {
                    invalid_ids.push(val);
                }
            }
        }

        let sum_of_invalid_ids:u64 = invalid_ids.iter().sum::<u64>();

        assert_eq!(sum_of_invalid_ids, 1227775554);
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn id_of_11_is_invalid() {
        let id_range = IdRange::new(11, 22);
        assert!(id_range.is_invalid_id_part2(11));
    }

    #[test]
    fn id_of_22_is_invalid() {
        let id_range = IdRange::new(11, 22);
        assert!(id_range.is_invalid_id_part2(22));
    }

    #[test]
    fn id_of_99_is_invalid() {
        let id_range = IdRange::new(95, 115);
        assert!(id_range.is_invalid_id_part2(99));
    }

    #[test]
    fn id_of_111_is_invalid() {
        let id_range = IdRange::new(95, 115);
        assert!(id_range.is_invalid_id_part2(111));
    }

    #[test]
    fn id_of_1010_is_invalid() {
        let id_range = IdRange::new(998, 1012);
        assert!(id_range.is_invalid_id_part2(1010));
    }

    #[test]
    fn id_of_1188511885_is_invalid() {
        let id_range = IdRange::new(1188511880, 1188511890);
        assert!(id_range.is_invalid_id_part2(1188511885));
    }

    #[test]
    fn id_of_222222_is_invalid() {
        let id_range = IdRange::new(222220, 222224);
        assert!(id_range.is_invalid_id_part2(222222));
    }

    #[test]
    fn id_of_446446_is_invalid() {
        let id_range = IdRange::new(446443, 446449);
        assert!(id_range.is_invalid_id_part2(446446));
    }

    #[test]
    fn id_of_38593859_is_invalid() {
        let id_range = IdRange::new(38593856, 38593862);
        assert!(id_range.is_invalid_id_part2(38593859));
    }

    #[test]
    fn id_of_565656_is_invalid() {
        let id_range = IdRange::new(565653, 565659);
        assert!(id_range.is_invalid_id_part2(565656));
    }

    #[test]
    fn id_of_824824824_is_invalid() {
        let id_range = IdRange::new(824824821, 824824827);
        assert!(id_range.is_invalid_id_part2(824824824));
    }

    #[test]
    fn id_of_2121212121_is_invalid() {
        let id_range = IdRange::new(2121212118, 2121212124);
        assert!(id_range.is_invalid_id_part2(2121212121));
    }

    #[test]
    fn range_of_11_to_22_has_two_invalid_ids() {
        let range = IdRange::new(11, 22);

        let mut invalid_count = 0;

        for val in range.min..=range.max {
            if range.is_invalid_id_part2(val) {
                invalid_count += 1;
            }
        }

        assert_eq!(invalid_count, 2);
    }

    #[test]
    fn range_of_998_to_1012_has_two_invalid_ids() {
        let range = IdRange::new(998, 1012);

        let mut invalid_count = 0;

        for val in range.min..=range.max {
            if range.is_invalid_id_part2(val) {
                invalid_count += 1;
            }
        }

        assert_eq!(invalid_count, 2);
    }

    #[test]
    fn range_of_1188511880_to_1188511890_has_one_invalid_id() {
        let range = IdRange::new(1188511880, 1188511890);

        let mut invalid_count = 0;

        for val in range.min..=range.max {
            if range.is_invalid_id_part2(val) {
                invalid_count += 1;
            }
        }

        assert_eq!(invalid_count, 1);
    }

    #[test]
    fn range_of_222220_to_222224_has_one_invalid_id() {
        let range = IdRange::new(222220, 222224);

        let mut invalid_count = 0;

        for val in range.min..=range.max {
            if range.is_invalid_id_part2(val) {
                invalid_count += 1;
            }
        }

        assert_eq!(invalid_count, 1);
    }

    #[test]
    fn range_of_1698522_to_1698528_has_no_invalid_ids() {
        let range = IdRange::new(1698522, 1698528);

        let mut invalid_count = 0;

        for val in range.min..=range.max {
            if range.is_invalid_id_part2(val) {
                invalid_count += 1;
            }
        }

        assert_eq!(invalid_count, 0);
    }

    #[test]
    fn range_of_446443_to_446449_has_one_invalid_id() {
        let range = IdRange::new(446443, 446449);

        let mut invalid_count = 0;

        for val in range.min..=range.max {
            if range.is_invalid_id_part2(val) {
                invalid_count += 1;
            }
        }

        assert_eq!(invalid_count, 1);
    }

    #[test]
    fn range_of_38593856_to_38593862_has_one_invalid_id() {
        let range = IdRange::new(38593856, 38593862);

        let mut invalid_count = 0;

        for val in range.min..=range.max {
            if range.is_invalid_id_part2(val) {
                invalid_count += 1;
            }
        }

        assert_eq!(invalid_count, 1);
    }

    #[test]
    fn range_of_565653_to_565659_has_one_invalid_id() {
        let range = IdRange::new(565653, 565659);

        let mut invalid_count = 0;

        for val in range.min..=range.max {
            if range.is_invalid_id_part2(val) {
                invalid_count += 1;
            }
        }

        assert_eq!(invalid_count, 1);
    }

    #[test]
    fn range_of_824824821_to_824824827_has_one_invalid_id() {
        let range = IdRange::new(824824821, 824824827);

        let mut invalid_count = 0;

        for val in range.min..=range.max {
            if range.is_invalid_id_part2(val) {
                invalid_count += 1;
            }
        }

        assert_eq!(invalid_count, 1);
    }

    #[test]
    fn range_of_2121212118_to_2121212124_has_one_invalid_id() {
        let range = IdRange::new(2121212118, 2121212124);

        let mut invalid_count = 0;

        for val in range.min..=range.max {
            if range.is_invalid_id_part2(val) {
                invalid_count += 1;
            }
        }

        assert_eq!(invalid_count, 1);
    }

    #[test]
    fn sum_of_invalid_ids_in_test_input_is_4174379265() {
        let split_ranges = split_input_into_id_ranges(Path::new("input/test_input.txt").into());

        let mut invalid_ids:Vec<u64> = Vec::new();

        for id_range in split_ranges {
            for val in id_range.min..=id_range.max {
                if id_range.is_invalid_id_part2(val) {
                    invalid_ids.push(val);
                }
            }
        }

        let sum_of_invalid_ids:u64 = invalid_ids.iter().sum::<u64>();

        assert_eq!(sum_of_invalid_ids, 4174379265);
    }
}

