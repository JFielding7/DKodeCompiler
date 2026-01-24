use string_interner::DefaultSymbol;
use crate::phase::Phase;
use crate::source::source_span::SourceSpan;

#[derive(Debug)]
pub struct Symbol<T: Phase> {
    pub name: DefaultSymbol,
    pub symbol_type: SymbolType,
    pub def_span: SourceSpan,
    pub data_type_id: T::SymbolDataTypeId,
    pub llvm_var: T::LLVMVariable,
}

impl<T: Phase> Symbol<T> {
    pub fn new(
        name: DefaultSymbol,
        symbol_type: SymbolType,
        def_span: SourceSpan,
        data_type_id: T::SymbolDataTypeId,
        llvm_var: T::LLVMVariable,
    ) -> Self {
        Self {
            name,
            def_span,
            symbol_type,
            data_type_id,
            llvm_var,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SymbolType {
    Variable,
    FunctionParam(usize),
    ClassField(usize),
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct SymbolId(usize);

impl SymbolId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}
