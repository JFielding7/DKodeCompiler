#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum UnaryOperator {
    Neg,
    Not,
    BitNot,
    PreInc,
    PreDec,
    PostInc,
    PostDec,
}

impl std::fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use UnaryOperator::*;
        
        write!(f, "{}", match self {
            Neg => "-",
            Not => "!",
            BitNot => "~",
            PreInc => "++",
            PreDec => "--",
            PostInc => "++",
            PostDec => "--",
        })
    }
}
