use OperatorPrecedenceGroup::*;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OperatorPrecedenceGroup {
    Comma = 0,
    Assign,
    LogicalOr,
    LogicalAnd,
    BitOr,
    BitXor,
    BitAnd,
    Equality,
    Relational,
    BitShift,
    Add,
    Mul,
    Prefix,
    Postfix,
}

impl OperatorPrecedenceGroup {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn binding_power(self) -> (u8, u8) {
        let binding_power = self.as_u8();
        (binding_power, binding_power + (self.is_left_assoc() as u8))
    }

    pub fn is_left_assoc(self) -> bool {
        !matches!(self, Assign | Prefix)
    }
}
