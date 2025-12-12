#[derive(Debug)]
pub struct DialRotation {
    direction:char,
    rotation_amount:i32,
}

impl DialRotation {
    pub fn new(direction:char, rotation_amount:i32) -> DialRotation {
        DialRotation {direction, rotation_amount }
    }

    pub fn get_direction(&self) -> char {
        self.direction
    }

    pub fn get_value(&self) -> i32 {
        self.rotation_amount
    }
}

pub struct Dial {
    current_position: i32,
    zero_count:i32
}

impl Dial {
    pub fn new(current_position:i32, zero_count:i32) -> Result<Dial, String> {
        if current_position > 99 {
            return Err("Invalid dial starting position".to_string());
        }
        Ok(Dial {current_position, zero_count})
    }

    pub fn get_current_position(&self) -> i32 {
        self.current_position
    }

    pub fn get_zero_count(&self) -> i32 {
        self.zero_count
    }

    pub fn turn_dial_in_direction_part1(&mut self, dial_rotation: &DialRotation) {
        self.increment_zero_count_if_dial_at_zero();

        match dial_rotation.get_direction() {
            'L' => {
                self.set_dial_to_new_position((self.current_position - dial_rotation.get_value()).rem_euclid(100));
            },
            'R' => {
                self.set_dial_to_new_position((self.current_position + dial_rotation.get_value()).rem_euclid(100));
            },
            _ => panic!("Invalid dial rotation (this should absolutely never happen given the input)")
        }
    }

    pub fn turn_dial_in_direction_part2(&mut self, dial_rotation: &DialRotation) {
        self.increment_zero_count_if_dial_at_zero();

        println!("Current dial: {}{}", dial_rotation.direction, dial_rotation.rotation_amount);
        match dial_rotation.get_direction() {
            'L' => {
                self.increment_zero_count_by_full_rotations(dial_rotation);
                self.increment_zero_count_if_dial_rotates_past_zero(dial_rotation);
                self.set_dial_to_new_position((self.current_position - dial_rotation.get_value()).rem_euclid(100));
            },
            'R' => {
                self.increment_zero_count_by_full_rotations(dial_rotation);
                self.increment_zero_count_if_dial_rotates_past_zero(dial_rotation);
                self.set_dial_to_new_position((self.current_position + dial_rotation.get_value()).rem_euclid(100));
            },
            _ => panic!("Invalid dial rotation (this should absolutely never happen given the input)")
        }
    }

    pub fn set_dial_to_new_position(&mut self, new_position:i32) { self.current_position = new_position; }

    pub fn count_full_rotations(&self, dial_rotation: &DialRotation) -> i32 {
        (dial_rotation.get_value() as f32 / 100.0f32).floor() as i32
    }

    pub fn check_if_dial_rotates_past_zero(&self, dial_rotation: &DialRotation) -> bool {
        if self.current_position == 0 { return false; }

        let value_after_full_rotations = dial_rotation.get_value().rem_euclid(100);

        match dial_rotation.get_direction() {
            'L' => self.current_position - value_after_full_rotations < 0,
            'R' => self.current_position + value_after_full_rotations > 100,
            _ => panic!("Invalid dial rotation (this should absolutely never happen given the input)")
        }
    }

    pub fn dial_is_at_zero(&mut self) -> bool { self.current_position == 0 }

    pub fn increment_zero_count_if_dial_at_zero(&mut self) {
        if self.dial_is_at_zero() {
            self.zero_count += 1;
        }
    }

    pub fn increment_zero_count_by_full_rotations(&mut self, dial_rotation: &DialRotation) {
        self.zero_count += self.count_full_rotations(dial_rotation);

    }

    pub fn increment_zero_count_if_dial_rotates_past_zero(&mut self, dial_rotation: &DialRotation) {
        if self.check_if_dial_rotates_past_zero(dial_rotation) {
            self.zero_count += 1;
        }
    }

}

pub fn get_dial_rotation(dial_rotation: &str) -> DialRotation {
    DialRotation {
        direction:dial_rotation.chars().nth(0).unwrap(),
        rotation_amount:dial_rotation.chars().skip(1).collect::<String>().parse::<i32>().unwrap()
    }
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn getting_dial_rotation_input_works_part1() {
        let dial_rotation = get_dial_rotation("L68");
        assert_eq!(dial_rotation.direction, 'L');
        assert_eq!(dial_rotation.rotation_amount, 68);
    }

    #[test]
    fn new_dial_over_99_should_not_work_part1() {
        let dial = Dial::new(100, 0);
        assert!(dial.is_err());
    }

    #[test]
    fn new_dial_at_0_should_work_part1() {
        let dial = Dial::new(0, 0);
        assert!(dial.is_ok());
    }

    #[test]
    fn new_dial_under_99_should_work_part1() {
        let dial = Dial::new(99, 0);
        assert!(dial.is_ok());
    }

    #[test]
    fn turning_dial_left_by_5_results_in_45_part1() {
        let dial_rotation = get_dial_rotation("L5");
        let mut dial = Dial::new(50, 0).unwrap();

        dial.turn_dial_in_direction_part1(&dial_rotation);
        let dial_result_position = dial.get_current_position();

        assert_eq!(dial_result_position, 45);
    }

    #[test]
    fn turning_dial_right_by_5_results_in_55_part1() {
        let dial_rotation = get_dial_rotation("R5");
        let mut dial = Dial::new(50, 0).unwrap();

        dial.turn_dial_in_direction_part1(&dial_rotation);
        let dial_result_position = dial.get_current_position();

        assert_eq!(dial_result_position, 55);
    }

    #[test]
    fn turning_dial_left_by_1_from_0_results_in_99_part1() {
        let dial_rotation = get_dial_rotation("L1");
        let mut dial = Dial::new(0, 0).unwrap();

        dial.turn_dial_in_direction_part1(&dial_rotation);
        let dial_result_position = dial.get_current_position();

        assert_eq!(dial_result_position, 99);
    }

    #[test]
    fn turning_dial_right_by_1_from_99_results_in_0_part1() {
        let dial_rotation = get_dial_rotation("R1");
        let mut dial = Dial::new(99, 0).unwrap();

        dial.turn_dial_in_direction_part1(&dial_rotation);
        let dial_result_position = dial.get_current_position();

        assert_eq!(dial_result_position, 0);
    }

    #[test]
    fn turning_dial_left_by_68_from_50_results_in_82_part1() {
        let dial_rotation = get_dial_rotation("L68");
        let mut dial = Dial::new(50, 0).unwrap();

        dial.turn_dial_in_direction_part1(&dial_rotation);
        let dial_result_position = dial.get_current_position();

        assert_eq!(dial_result_position, 82);
    }

    #[test]
    fn turning_dial_right_by_48_from_52_results_in_0_part1() {
        let dial_rotation = get_dial_rotation("R48");
        let mut dial = Dial::new(52, 0).unwrap();

        dial.turn_dial_in_direction_part1(&dial_rotation);
        let dial_result_position = dial.get_current_position();

        assert_eq!(dial_result_position, 0);
    }

    #[test]
    fn turning_dial_right_by_300_from_50_results_in_50_part1() {
        let dial_rotation = get_dial_rotation("R300");
        let mut dial = Dial::new(50, 0).unwrap();

        dial.turn_dial_in_direction_part1(&dial_rotation);
        let dial_result_position = dial.get_current_position();

        assert_eq!(dial_result_position, 50);
    }

}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn turning_dial_right_by_300_from_50_results_in_50_part2() {
        let dial_rotation = get_dial_rotation("R300");
        let mut dial = Dial::new(50, 0).unwrap();

        dial.turn_dial_in_direction_part2(&dial_rotation);
        let dial_result_position = dial.get_current_position();

        assert_eq!(dial_result_position, 50);
    }

    #[test]
    fn turning_dial_left_by_300_from_10_results_in_10_part2() {
        let dial_rotation = get_dial_rotation("L300");
        let mut dial = Dial::new(10, 0).unwrap();

        dial.turn_dial_in_direction_part2(&dial_rotation);
        let dial_result_position = dial.get_current_position();

        assert_eq!(dial_result_position, 10);
    }

    #[test]
    fn turning_dial_left_by_341_from_10_results_in_69_part2() {
        let dial_rotation = get_dial_rotation("L341");
        let mut dial = Dial::new(10, 0).unwrap();

        dial.turn_dial_in_direction_part2(&dial_rotation);
        let dial_result_position = dial.get_current_position();

        assert_eq!(dial_result_position, 69);
    }

    #[test]
    fn turning_dial_left_by_50_at_99_results_in_49_part2() {
        let dial_rotation = get_dial_rotation("L50");
        let mut dial = Dial::new(99, 0).unwrap();

        dial.turn_dial_in_direction_part2(&dial_rotation);
        let dial_result_position = dial.get_current_position();

        assert_eq!(dial_result_position, 49);
    }

    #[test]
    fn turning_dial_right_by_541_from_0_results_in_41_part2() {
        let dial_rotation = get_dial_rotation("R541");
        let mut dial = Dial::new(0, 0).unwrap();

        dial.turn_dial_in_direction_part2(&dial_rotation);
        let dial_result_position = dial.get_current_position();

        assert_eq!(dial_result_position, 41);
    }
    
    #[test]
    fn turning_dial_right_by_399_from_50_should_pass_zero_4_times_part2() {
        let dial_rotation = get_dial_rotation("R399");
        let mut dial = Dial::new(50, 0).unwrap();

        dial.increment_zero_count_by_full_rotations(&dial_rotation);
        dial.increment_zero_count_if_dial_rotates_past_zero(&dial_rotation);

        assert_eq!(dial.get_zero_count(), 4);
    }

    #[test]
    fn turning_dial_right_by_400_from_50_should_pass_zero_4_times_part2() {
        let dial_rotation = get_dial_rotation("R400");
        let mut dial = Dial::new(50, 0).unwrap();

        dial.increment_zero_count_by_full_rotations(&dial_rotation);
        dial.increment_zero_count_if_dial_rotates_past_zero(&dial_rotation);

        assert_eq!(dial.get_zero_count(), 4);
    }

    #[test]
    fn turning_dial_right_499_from_99_should_pass_zero_5_times_part2() {
        let dial_rotation = get_dial_rotation("R499");
        let mut dial = Dial::new(99, 0).unwrap();

        dial.increment_zero_count_by_full_rotations(&dial_rotation);
        dial.increment_zero_count_if_dial_rotates_past_zero(&dial_rotation);

        assert_eq!(dial.get_zero_count(), 5);
    }

    #[test]
    fn turning_dial_left_by_300_from_10_should_pass_zero_3_times_part2() {
        let dial_rotation = get_dial_rotation("L300");
        let mut dial = Dial::new(10, 0).unwrap();

        dial.increment_zero_count_by_full_rotations(&dial_rotation);
        dial.increment_zero_count_if_dial_rotates_past_zero(&dial_rotation);

        assert_eq!(dial.get_zero_count(), 3);
    }

    #[test]
    fn turning_dial_left_by_341_from_10_should_pass_zero_4_times_part2() {
        let dial_rotation = get_dial_rotation("L341");
        let mut dial = Dial::new(10, 0).unwrap();

        dial.increment_zero_count_by_full_rotations(&dial_rotation);
        dial.increment_zero_count_if_dial_rotates_past_zero(&dial_rotation);

        assert_eq!(dial.get_zero_count(), 4);
    }

    #[test]
    fn turning_dial_left_50_from_99_should_pass_zero_0_times_part2() {
        let dial_rotation = get_dial_rotation("L50");
        let mut dial = Dial::new(99, 0).unwrap();

        dial.increment_zero_count_by_full_rotations(&dial_rotation);
        dial.increment_zero_count_if_dial_rotates_past_zero(&dial_rotation);

        assert_eq!(dial.get_zero_count(), 0);
    }

    #[test]
    fn turning_dial_left_by_5_from_0_should_pass_zero_0_times_part2() {
        let dial_rotation = get_dial_rotation("L5");
        let mut dial = Dial::new(0, 0).unwrap();

        dial.increment_zero_count_by_full_rotations(&dial_rotation);
        dial.increment_zero_count_if_dial_rotates_past_zero(&dial_rotation);

        assert_eq!(dial.get_zero_count(), 0);
    }
}


