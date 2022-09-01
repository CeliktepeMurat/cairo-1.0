use super::non_zero::NonZeroType;
use crate::extensions::lib_func::{
    LibFuncSignature, SignatureOnlyConcreteLibFunc, SpecializationContext,
};
use crate::extensions::{
    ConcreteType, GenericLibFunc, NamedLibFunc, NamedType, NoGenericArgsGenericLibFunc,
    NoGenericArgsGenericType, SignatureBasedConcreteLibFunc, SpecializationError,
};
use crate::ids::{GenericLibFuncId, GenericTypeId};
use crate::program::GenericArg;
use crate::{define_concrete_libfunc_hierarchy, define_libfunc_hierarchy};

/// Type for int.
#[derive(Default)]
pub struct IntegerType {}
impl NoGenericArgsGenericType for IntegerType {
    type Concrete = IntegerConcreteType;
    const ID: GenericTypeId = GenericTypeId::new_inline("int");
}
#[derive(Default)]
pub struct IntegerConcreteType {}
impl ConcreteType for IntegerConcreteType {}

define_libfunc_hierarchy! {
    pub enum IntegerLibFunc {
        Operation(OperationLibFunc),
        Const(ConstLibFunc),
        Ignore(IgnoreLibFunc),
        Duplicate(DuplicateLibFunc),
    }, IntegerConcrete
}

/// Possible operators for integers.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

/// Libfunc for operations on integers.
pub struct OperationLibFunc {
    pub operator: Operator,
}
impl GenericLibFunc for OperationLibFunc {
    type Concrete = OperationConcreteLibFunc;
    fn by_id(id: &GenericLibFuncId) -> Option<Self> {
        const INT_ADD: GenericLibFuncId = GenericLibFuncId::new_inline("int_add");
        const INT_SUB: GenericLibFuncId = GenericLibFuncId::new_inline("int_sub");
        const INT_MUL: GenericLibFuncId = GenericLibFuncId::new_inline("int_mul");
        const INT_DIV: GenericLibFuncId = GenericLibFuncId::new_inline("int_div");
        const INT_MOD: GenericLibFuncId = GenericLibFuncId::new_inline("int_mod");
        match id {
            id if id == &INT_ADD => Some(OperationLibFunc { operator: Operator::Add }),
            id if id == &INT_SUB => Some(OperationLibFunc { operator: Operator::Sub }),
            id if id == &INT_MUL => Some(OperationLibFunc { operator: Operator::Mul }),
            id if id == &INT_DIV => Some(OperationLibFunc { operator: Operator::Div }),
            id if id == &INT_MOD => Some(OperationLibFunc { operator: Operator::Mod }),
            _ => None,
        }
    }
    fn specialize(
        &self,
        context: SpecializationContext<'_>,
        args: &[GenericArg],
    ) -> Result<Self::Concrete, SpecializationError> {
        let int_type = context.get_concrete_type(IntegerType::id(), &[])?;
        match args {
            [] => Ok(OperationConcreteLibFunc::Binary(BinaryOperationConcreteLibFunc {
                operator: self.operator,
                signature: LibFuncSignature::non_branch(
                    vec![
                        int_type.clone(),
                        if matches!(self.operator, Operator::Div | Operator::Mod) {
                            context
                                .get_wrapped_concrete_type(NonZeroType::id(), int_type.clone())?
                        } else {
                            int_type.clone()
                        },
                    ],
                    vec![int_type],
                ),
            })),
            [GenericArg::Value(c)] => {
                if matches!(self.operator, Operator::Div | Operator::Mod) && *c == 0 {
                    Err(SpecializationError::UnsupportedGenericArg)
                } else {
                    Ok(OperationConcreteLibFunc::Const(OperationWithConstConcreteLibFunc {
                        operator: self.operator,
                        c: *c,
                        signature: LibFuncSignature::non_branch(
                            vec![int_type.clone()],
                            vec![int_type],
                        ),
                    }))
                }
            }
            _ => Err(SpecializationError::UnsupportedGenericArg),
        }
    }
}

pub struct BinaryOperationConcreteLibFunc {
    pub operator: Operator,
    pub signature: LibFuncSignature,
}
impl SignatureBasedConcreteLibFunc for BinaryOperationConcreteLibFunc {
    fn signature(&self) -> &LibFuncSignature {
        &self.signature
    }
}

/// Operations between a int and a const.
pub struct OperationWithConstConcreteLibFunc {
    pub operator: Operator,
    pub c: i64,
    pub signature: LibFuncSignature,
}
define_concrete_libfunc_hierarchy! {
    pub enum OperationConcreteLibFunc {
        Binary(BinaryOperationConcreteLibFunc),
        Const(OperationWithConstConcreteLibFunc),
    }
}

impl SignatureBasedConcreteLibFunc for OperationWithConstConcreteLibFunc {
    fn signature(&self) -> &LibFuncSignature {
        &self.signature
    }
}

/// LibFunc for creating a constant int.
#[derive(Default)]
pub struct ConstLibFunc {}
impl NamedLibFunc for ConstLibFunc {
    type Concrete = ConstConcreteLibFunc;
    const ID: GenericLibFuncId = GenericLibFuncId::new_inline("int_const");
    fn specialize(
        &self,
        context: SpecializationContext<'_>,
        args: &[GenericArg],
    ) -> Result<Self::Concrete, SpecializationError> {
        match args {
            [GenericArg::Value(c)] => Ok(ConstConcreteLibFunc {
                c: *c,
                signature: LibFuncSignature::non_branch(
                    vec![],
                    vec![context.get_concrete_type(IntegerType::id(), &[])?],
                ),
            }),
            _ => Err(SpecializationError::UnsupportedGenericArg),
        }
    }
}

pub struct ConstConcreteLibFunc {
    pub c: i64,
    pub signature: LibFuncSignature,
}
impl SignatureBasedConcreteLibFunc for ConstConcreteLibFunc {
    fn signature(&self) -> &LibFuncSignature {
        &self.signature
    }
}

/// LibFunc for ignoring an int.
#[derive(Default)]
pub struct IgnoreLibFunc {}
impl NoGenericArgsGenericLibFunc for IgnoreLibFunc {
    type Concrete = SignatureOnlyConcreteLibFunc;
    const ID: GenericLibFuncId = GenericLibFuncId::new_inline("int_ignore");
    fn specialize(
        &self,
        context: SpecializationContext<'_>,
    ) -> Result<Self::Concrete, SpecializationError> {
        Ok(SignatureOnlyConcreteLibFunc {
            signature: LibFuncSignature::non_branch(
                vec![context.get_concrete_type(IntegerType::id(), &[])?],
                vec![],
            ),
        })
    }
}

/// LibFunc for duplicating an int.
#[derive(Default)]
pub struct DuplicateLibFunc {}
impl NoGenericArgsGenericLibFunc for DuplicateLibFunc {
    type Concrete = SignatureOnlyConcreteLibFunc;
    const ID: GenericLibFuncId = GenericLibFuncId::new_inline("int_dup");

    fn specialize(
        &self,
        context: SpecializationContext<'_>,
    ) -> Result<Self::Concrete, SpecializationError> {
        let int_type = context.get_concrete_type(IntegerType::id(), &[])?;
        Ok(SignatureOnlyConcreteLibFunc {
            signature: LibFuncSignature::non_branch(
                vec![int_type.clone(), int_type.clone()],
                vec![int_type],
            ),
        })
    }
}