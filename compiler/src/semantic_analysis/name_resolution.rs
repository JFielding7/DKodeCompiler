pub mod name_resolver;
mod symbol_table;

use std::collections::HashMap;
use string_interner::DefaultSymbol;
use crate::phase::Phase;
use crate::phase::symbol_table::symbol::Symbol;

#[derive(Debug, Default)]
pub struct NameResolution;

impl Phase for NameResolution {
    type Symbols = HashMap<DefaultSymbol, Symbol<NameResolution>>;
    type UnaryOpImpl = ();
    type BinaryOpImpl = ();
    type SymbolDataTypeId = ();
    type DataTypeRepr = ();
    type VariableRepr = ();
    type FunctionRepr = ();
}
