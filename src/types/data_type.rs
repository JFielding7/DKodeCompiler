use strum::EnumIter;
use string_interner::DefaultSymbol;

#[derive(Debug, Clone)]
pub enum DataType {
    Builtin(BuiltinType),
    UserDefined(DefaultSymbol),
    // TODO: generics
}

#[derive(Debug, Clone, PartialEq, EnumIter)]
pub enum BuiltinType {
    Unit = 0,
    Bool,
    Int,
    String,
}
