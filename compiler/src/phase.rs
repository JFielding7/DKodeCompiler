use std::fmt::Debug;

pub mod symbol_table;
pub mod types;


pub trait Phase: Debug {
    type Symbols: Debug;
    type UnaryOpImpl: Debug;
    type BinaryOpImpl: Debug;
    type SymbolDataTypeId: Debug;
    type DataTypeRepr: Debug;
    type VariableRepr: Debug;
    type FunctionRepr: Debug;
}

pub trait MultiPhase {
    type LastPhase: Phase;
}

#[derive(Debug)]
pub struct SyntaxAnalysis;

impl Phase for SyntaxAnalysis {
    type Symbols = ();
    type UnaryOpImpl = ();
    type BinaryOpImpl = ();
    type SymbolDataTypeId = ();
    type DataTypeRepr = ();
    type VariableRepr = ();
    type FunctionRepr = ();
}
