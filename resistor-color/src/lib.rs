use enum_iterator::all;
use enum_iterator::Sequence;
use int_enum::IntEnum;

#[repr(usize)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color.int_value()
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value) {
        Ok(resistor_color) => format!("{:?}", resistor_color),
        Err(_) => format!("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut vec = all::<ResistorColor>().collect::<Vec<_>>();
    vec.sort_by(|a, b| color_to_value(*a).cmp(&color_to_value(*b)));
    vec
}
