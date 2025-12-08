use bevy::input::keyboard::Key;
use smol_str::SmolStr;

pub const DIGIT_KEYS: [Key; 10] = [
    Key::Character(SmolStr::new_inline("0")),
    Key::Character(SmolStr::new_inline("1")),
    Key::Character(SmolStr::new_inline("2")),
    Key::Character(SmolStr::new_inline("3")),
    Key::Character(SmolStr::new_inline("4")),
    Key::Character(SmolStr::new_inline("5")),
    Key::Character(SmolStr::new_inline("6")),
    Key::Character(SmolStr::new_inline("7")),
    Key::Character(SmolStr::new_inline("8")),
    Key::Character(SmolStr::new_inline("9")),
];

#[derive(PartialEq, Eq)]
pub enum InputMode {
    Mouse,
    Keyboard,
}
