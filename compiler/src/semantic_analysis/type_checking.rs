pub mod type_checker;
mod symbol_table;
mod types;

use std::collections::HashMap;
use string_interner::DefaultSymbol;
use crate::operators::binary_operators::BinaryOperator;
use crate::operators::unary_operators::UnaryOperator;
use crate::phase::Phase;
use crate::phase::symbol_table::operator_registry::OperatorRegistry;
use crate::phase::symbol_table::symbol::Symbol;
use crate::phase::types::data_type::DataTypeId;

#[derive(Debug, PartialEq, Default)]
pub struct TypeChecking;

impl Phase for TypeChecking {
    type Symbols = HashMap<DefaultSymbol, Symbol<TypeChecking>>;
    type UnaryOpImpl = OperatorRegistry<TypeChecking, UnaryOperator>;
    type BinaryOpImpl = OperatorRegistry<TypeChecking, BinaryOperator>;
    type SymbolDataTypeId = DataTypeId;
    type DataTypeRepr = ();
    type VariableRepr = ();
    type FunctionRepr = ();
}