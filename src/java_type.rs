use core::panicking::panic;
use std::str::Chars;
use std::thread::park;

pub enum Type {
    Void,
    Boolean,
    Byte,
    Short,
    Char,
    Int,
    Float,
    Long,
    Double,
    Class { name: String },
    Array { component: Type },
}

impl Type {
    pub fn parse_str(text: &mut String, start: usize, stop: usize) -> Type {
        Self::parse(&mut text.chars(), start, stop)
    }

    pub fn parse(chars: &mut Chars, start: usize, stop: usize) -> Type {
        let first_char = chars.nth(current_pos).unwrap();
        return match first_char {
            'B' => Type::Byte,
            'C' => Type::Char,
            'D' => Type::Double,
            'F' => Type::Float,
            'I' => Type::Int,
            'J' => Type::Long,
            'S' => Type::Short,
            'Z' => Type::Boolean,
            '[' => Type::Array { component: Self::parse(chars, start + 1, stop) },
            'L' => {
                let stop = chars.position(|c| c == ';').unwrap() - 1;
                Type::Class {
                    name: chars.as_str()[(start + 1)..stop].to_string()
                }
            }
            _ => panic!("Invalid Type Parsing")
        };
    }
}