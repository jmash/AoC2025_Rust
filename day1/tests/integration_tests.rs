use day1::*;
use std::fs::File;
use std::fs::read_to_string;

#[test]
fn test_input_file_can_be_opened() {
    let test_input = File::open("input/test_input.txt");

    assert!(test_input.is_ok());
}

#[test]
fn test_input_contents_are_correct() {
    let correct_test_input =
        "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

    assert_eq!(correct_test_input, read_to_string("input/test_input.txt").unwrap());
}

// ---- Part 1 ----
#[test]
fn processing_test_input_results_in_3() {
    let mut dial = Dial::new(50, 0).unwrap();

    let test_input = read_to_string("input/test_input.txt").unwrap();

    let dial_rotation_strings = test_input.split('\n').collect::<Vec<&str>>();

    let mut dial_rotations:Vec<DialRotation> = vec![];

    for s in dial_rotation_strings {
        let dial_rotation = get_dial_rotation(s);
        dial_rotations.push(dial_rotation);
    }

    for r in dial_rotations {
        dial.turn_dial_in_direction(&r);
        dial.increment_zero_count_if_dial_at_zero();
    }

    assert_eq!(3, dial.get_zero_count());
}

#[test]
fn processing_puzzle_input_results_in_1118() {
    let mut dial = Dial::new(50, 0).unwrap();

    let test_input = read_to_string("input/input.txt").unwrap();

    let dial_rotation_strings = test_input.split('\n').collect::<Vec<&str>>();

    let mut dial_rotations:Vec<DialRotation> = vec![];

    for s in dial_rotation_strings {
        let dial_rotation = get_dial_rotation(s);
        dial_rotations.push(dial_rotation);
    }

    for r in dial_rotations {
        dial.turn_dial_in_direction(&r);
        dial.increment_zero_count_if_dial_at_zero();
    }

    assert_eq!(1118, dial.get_zero_count());
}

// ---- Part 2 ----
#[test]
fn processing_test_input_results_in_6() {
    let mut dial = Dial::new(50, 0).unwrap();

    let test_input = read_to_string("input/test_input.txt").unwrap();

    let dial_rotation_strings = test_input.split('\n').collect::<Vec<&str>>();

    let mut dial_rotations:Vec<DialRotation> = vec![];

    for s in dial_rotation_strings {
        let dial_rotation = get_dial_rotation(s);
        dial_rotations.push(dial_rotation);
    }

    for r in dial_rotations {
        dial.increment_zero_count_if_dial_passes_zero(&r);
        dial.increment_zero_count_if_dial_at_zero();
        dial.turn_dial_in_direction(&r);
    }

    assert_eq!(6, dial.get_zero_count());
}
