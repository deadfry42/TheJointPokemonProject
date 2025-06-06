use std::fmt;
use std::ops::Add;

#[allow(non_camel_case_types, dead_code)]
#[derive(Copy, Clone)]
pub enum MovePriority {
    Switching = 127,
    Item = 126,
    QuickClaw = 120,
    Bracket5 = 5,
    Bracket4 = 4,
    Bracket3 = 3,
    Bracket2 = 2,
    Bracket1 = 1,
    Neutral = 0,
    Bracket_1 = -1,
    Bracket_2 = -2,
    Bracket_3 = -3,
    Bracket_4 = -4,
    Bracket_5 = -5,
    Bracket_6 = -6,
    Bracket_7 = -7,
}

impl MovePriority {
    fn get_numberic_value(val: &MovePriority) -> i16 {
        *val as i16
    }

    fn get_enum_value(val: &i16) -> MovePriority {
        // this is the best way to do this i think
        match val {
            127 => MovePriority::Switching,
            126 => MovePriority::Item,
            120 => MovePriority::QuickClaw,
            5 => MovePriority::Bracket5,
            4 => MovePriority::Bracket4,
            3 => MovePriority::Bracket3,
            2 => MovePriority::Bracket2,
            1 => MovePriority::Bracket1,
            0 => MovePriority::Neutral,
            -1 => MovePriority::Bracket_1,
            -2 => MovePriority::Bracket_2,
            -3 => MovePriority::Bracket_3,
            -4 => MovePriority::Bracket_4,
            -5 => MovePriority::Bracket_5,
            -6 => MovePriority::Bracket_6,
            -7 => MovePriority::Bracket_7,
            _ => MovePriority::Neutral,
        }
    }
}

// priority + priority
impl Add for MovePriority {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        MovePriority::get_enum_value(
            &(MovePriority::get_numberic_value(&self) + MovePriority::get_numberic_value(&other)),
        )
    }
}

impl fmt::Display for MovePriority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", MovePriority::get_numberic_value(self))
    }
}
