use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;
use inkwell::values::{FunctionValue, PointerValue};
use string_interner::DefaultSymbol;
use crate::operators::binary_operators::BinaryOperator;
use crate::operators::unary_operators::UnaryOperator;
use crate::phase::symbol_table::operator_registry::OperatorRegistry;
use crate::phase::symbol_table::symbol::Symbol;
use crate::phase::types::data_type::DataTypeId;
use crate::phase::types::llvm_type::LLVMDataType;

pub mod symbol_table;
pub mod types;


pub trait Phase: Debug {
    type Symbols: Debug;
    type UnaryOpImpl: Debug;
    type BinaryOpImpl: Debug;
    type SymbolDataTypeId: Debug;
    type LLVMDataType: Debug;
    type LLVMVariable: Debug;
    type LLVMFunction: Debug;
}

#[derive(Debug)]
pub struct SyntaxAnalysis;

impl Phase for SyntaxAnalysis {
    type Symbols = ();
    type UnaryOpImpl = ();
    type BinaryOpImpl = ();
    type SymbolDataTypeId = ();
    type LLVMDataType = ();
    type LLVMVariable = ();
    type LLVMFunction = ();
}

#[derive(Debug, Default)]
pub struct NameResolution;

impl Phase for NameResolution {
    type Symbols = HashMap<DefaultSymbol, Symbol<NameResolution>>;
    type UnaryOpImpl = ();
    type BinaryOpImpl = ();
    type SymbolDataTypeId = ();
    type LLVMDataType = ();
    type LLVMVariable = ();
    type LLVMFunction = ();
}

#[derive(Debug, PartialEq, Default)]
pub struct TypeChecking;

impl Phase for TypeChecking {
    type Symbols = HashMap<DefaultSymbol, Symbol<TypeChecking>>;
    type UnaryOpImpl = OperatorRegistry<TypeChecking, UnaryOperator>;
    type BinaryOpImpl = OperatorRegistry<TypeChecking, BinaryOperator>;
    type SymbolDataTypeId = DataTypeId;
    type LLVMDataType = ();
    type LLVMVariable = ();
    type LLVMFunction = ();
}

#[derive(Debug)]
pub struct CodeGeneration<'llvm_ctx> {
    _marker: PhantomData<&'llvm_ctx ()>
}

impl<'llvm_ctx> Phase for CodeGeneration<'llvm_ctx> {
    type Symbols = HashMap<DefaultSymbol, Symbol<CodeGeneration<'llvm_ctx>>>;
    type UnaryOpImpl = OperatorRegistry<CodeGeneration<'llvm_ctx>, UnaryOperator>;
    type BinaryOpImpl = OperatorRegistry<CodeGeneration<'llvm_ctx>, BinaryOperator>;
    type SymbolDataTypeId = DataTypeId;
    type LLVMDataType = LLVMDataType<'llvm_ctx>;
    type LLVMVariable = PointerValue<'llvm_ctx>;
    type LLVMFunction = FunctionValue<'llvm_ctx>;
}

