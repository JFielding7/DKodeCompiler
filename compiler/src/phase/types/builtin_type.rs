use std::fmt::Formatter;
use strum::EnumIter;

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum BuiltinType {
    Unit = 0,
    Bool,
    Int,
    Str,
}

impl BuiltinType {
    pub fn as_usize(self) -> usize {
        self as usize
    }

    pub fn from_str(s: &str) -> Option<Self> {
        use BuiltinType::*;

        Some(match s {
            "bool" => Bool,
            "int" => Int,
            "str" => Str,
            _ => return None
        })
    }
}

impl std::fmt::Display for BuiltinType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use BuiltinType::*;
        
        write!(f, "{}", match self {
            Unit => "unit",
            Bool => "bool",
            Int => "int",
            Str => "str",
        })
    }
}
