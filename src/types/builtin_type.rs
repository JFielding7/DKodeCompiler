use strum::EnumIter;

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum BuiltinType {
    Unit = 0,
    Bool,
    Int,
    String,
}

impl BuiltinType {
    pub fn as_usize(self) -> usize {
        self as usize
    }

    pub fn from_string(s: &str) -> Option<Self> {
        use BuiltinType::*;

        Some(match s {
            "bool" => Bool,
            "int" => Int,
            "str" => String,
            _ => return None
        })
    }
}
