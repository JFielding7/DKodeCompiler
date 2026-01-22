use crate::operators::binary_operators::BinaryOperator::*;
use crate::operators::precedence::OperatorPrecedenceGroup;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum BinaryOperator {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    LeftShiftAssign,
    RightShiftAssign,
    AndAssign,
    XorAssign,
    OrAssign,

    Add,
    Sub,
    Mul,
    Div,
    Mod,

    BitAnd,
    BitOr,
    BitXor,

    LeftShift,
    RightShift,

    Equal,
    NotEquals,
    LessThan,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual,

    LogicalAnd,
    LogicalOr,

    CommaOperator,
}

impl BinaryOperator {
    pub fn precedence_group(self) -> OperatorPrecedenceGroup {
        use BinaryOperator::*;

        match self {
            CommaOperator => OperatorPrecedenceGroup::Comma,

            Assign
            | AddAssign
            | SubAssign
            | MulAssign
            | DivAssign
            | ModAssign
            | LeftShiftAssign
            | RightShiftAssign
            | AndAssign
            | XorAssign
            | OrAssign => OperatorPrecedenceGroup::Assign,

            LogicalOr => OperatorPrecedenceGroup::LogicalOr,
            LogicalAnd => OperatorPrecedenceGroup::LogicalAnd,

            BitOr => OperatorPrecedenceGroup::BitOr,
            BitXor => OperatorPrecedenceGroup::BitXor,
            BitAnd => OperatorPrecedenceGroup::BitAnd,

            Equal
            | NotEquals => OperatorPrecedenceGroup::Equality,

            LessThan
            | LessOrEqual
            | GreaterThan
            | GreaterOrEqual => OperatorPrecedenceGroup::Relational,

            LeftShift
            | RightShift => OperatorPrecedenceGroup::BitShift,

            Add
            | Sub => OperatorPrecedenceGroup::Add,

            Mul
            | Div
            | Mod => OperatorPrecedenceGroup::Mul,
        }
    }
}


impl std::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Assign => "=",
            AddAssign => "+=",
            SubAssign => "-=",
            MulAssign => "*=",
            DivAssign => "/=",
            ModAssign => "%=",
            LeftShiftAssign => "<<=",
            RightShiftAssign => ">>=",
            AndAssign => "&=",
            XorAssign => "^=",
            OrAssign => "|=",

            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Mod => "%",

            BitAnd => "&",
            BitOr => "|",
            BitXor => "^",

            LeftShift => "<<",
            RightShift => ">>",

            Equal => "==",
            NotEquals => "!=",
            LessThan => "<",
            LessOrEqual => "<=",
            GreaterThan => ">",
            GreaterOrEqual => ">=",

            LogicalAnd => "&&",
            LogicalOr => "||",

            CommaOperator => ",",
        };

        f.write_str(s)
    }
}
