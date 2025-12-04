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
}

impl Dial {
    pub fn new(current_position:i32) -> Result<Dial, String> {
        if current_position > 99 {
            return Err("Invalid dial starting position".to_string());
        }
        Ok(Dial {current_position})
    }

    pub fn get_current_position(&self) -> i32 {
        self.current_position
    }

    pub fn turn_dial_in_direction(&mut self, dial_rotation: DialRotation) {
        match dial_rotation.get_direction() {
            'L' => {
                self.current_position = (100 + (self.current_position - dial_rotation.get_value())) % 100
            },
            'R' => {
                self.current_position = (self.current_position + dial_rotation.get_value()) % 100
            },
            _ => panic!("Invalid dial rotation (this should absolutely never happen given the input)")
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
mod tests {
    use super::*;

    #[test]
    fn getting_dial_rotation_input_works() {
        let dial_rotation = get_dial_rotation("L68");
        assert_eq!(dial_rotation.direction, 'L');
        assert_eq!(dial_rotation.rotation_amount, 68);
    }

    #[test]
    fn new_dial_over_99_should_not_work() {
        let dial = Dial::new(100);
        assert!(dial.is_err());
    }

    #[test]
    fn new_dial_at_0_should_work() {
        let dial = Dial::new(0);
        assert!(dial.is_ok());
    }

    #[test]
    fn new_dial_under_99_should_work() {
        let dial = Dial::new(99);
        assert!(dial.is_ok());
    }

    #[test]
    fn turning_dial_left_by_5_results_in_45() {
        let dial_rotation = get_dial_rotation("L5");
        let mut dial = Dial::new(50).unwrap();

        dial.turn_dial_in_direction(dial_rotation);
        let dial_result_position = dial.get_current_position();

        assert_eq!(dial_result_position, 45);
    }

    #[test]
    fn turning_dial_right_by_5_results_in_55() {
        let dial_rotation = get_dial_rotation("R5");
        let mut dial = Dial::new(50).unwrap();

        dial.turn_dial_in_direction(dial_rotation);
        let dial_result_position = dial.get_current_position();

        assert_eq!(dial_result_position, 55);
    }

    #[test]
    fn turning_dial_left_by_1_from_0_results_in_99() {
        let dial_rotation = get_dial_rotation("L1");
        let mut dial = Dial::new(0).unwrap();

        dial.turn_dial_in_direction(dial_rotation);
        let dial_result_position = dial.get_current_position();

        assert_eq!(dial_result_position, 99);
    }

    #[test]
    fn turning_dial_right_by_1_from_99_results_in_0() {
        let dial_rotation = get_dial_rotation("R1");
        let mut dial = Dial::new(99).unwrap();

        dial.turn_dial_in_direction(dial_rotation);
        let dial_result_position = dial.get_current_position();

        assert_eq!(dial_result_position, 0);
    }

    #[test]
    fn turning_dial_left_by_68_from_50_results_in_82() {
        let dial_rotation = get_dial_rotation("L68");
        let mut dial = Dial::new(50).unwrap();

        dial.turn_dial_in_direction(dial_rotation);
        let dial_result_position = dial.get_current_position();

        assert_eq!(dial_result_position, 82);
    }

    #[test]
    fn turning_dial_right_by_48_from_52_results_in_0() {
        let dial_rotation = get_dial_rotation("R48");
        let mut dial = Dial::new(52).unwrap();

        dial.turn_dial_in_direction(dial_rotation);
        let dial_result_position = dial.get_current_position();

        assert_eq!(dial_result_position, 0);
    }

    #[test]
    fn turning_dial_right_by_300_from_50_results_in_50() {
        let dial_rotation = get_dial_rotation("R300");
        let mut dial = Dial::new(50).unwrap();

        dial.turn_dial_in_direction(dial_rotation);
        let dial_result_position = dial.get_current_position();

        assert_eq!(dial_result_position, 50);
    }
}


