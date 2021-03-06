use std::f64::consts;

use cods_derive::{EnumDisplay, EnumFromStr};

use crate::ast::BuiltinFunCall::{self, *};
use crate::DataType::*;
use crate::{DataType, Val};

const PI: Val = Val::Float(consts::PI);
const TAU: Val = Val::Float(consts::TAU);
const E: Val = Val::Float(consts::E);

#[derive(Clone, Copy, Debug, PartialEq, EnumDisplay, EnumFromStr)]
#[cods(rename_all = "snake_case")]
pub enum BuiltinFun {
    Pow,
    Ln,
    Log,
    Sqrt,
    Ncr,
    ToDeg,
    ToRad,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Gcd,
    Min,
    Max,
    Clamp,
    Abs,
    Print,
    Println,
    Spill,
    SpillLocal,
    Assert,
    AssertEq,
}

#[derive(Clone, Copy, Debug, PartialEq, EnumDisplay, EnumFromStr)]
#[cods(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BuiltinConst {
    Pi,
    Tau,
    E,
}

impl BuiltinConst {
    pub const fn val(&self) -> Val {
        match self {
            Self::Pi => PI,
            Self::Tau => TAU,
            Self::E => E,
        }
    }

    pub const fn data_type(&self) -> DataType {
        match self {
            Self::Pi => PI.data_type(),
            Self::Tau => TAU.data_type(),
            Self::E => E.data_type(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunSignature {
    pub params: &'static [DataType],
    /// Repetition for the last parameter
    pub repetition: Repetition,
    pub return_type: DataType,
}

impl FunSignature {
    pub const fn new(
        params: &'static [DataType],
        repetition: Repetition,
        return_type: DataType,
    ) -> Self {
        Self {
            params,
            repetition,
            return_type,
        }
    }

    pub const fn empty() -> Self {
        Self::new(&[], Repetition::One, DataType::Unit)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Repetition {
    /// a: int
    One,
    /// a: ..int
    ZeroOrMore,
    /// a: ...int
    OneOrMore,
}

macro_rules! fun_signatures {
    ($($builtin:ident($($params:tt)*) -> $return_type:ident)*) => {{
        [
            $(fun_signature!($builtin($($params)*) -> $return_type)),*
        ]
    }};
}

macro_rules! fun_signature {
    ($builtin:ident($($params:ident),*) -> $return_type:ident) => {{
        (
            $builtin,
            FunSignature::new(
                &[$($params),*],
                Repetition::One,
                $return_type,
            )
        )
    }};
    ($builtin:ident($($params:ident,)* ..$last:ident) -> $return_type:ident) => {{
        (
            $builtin,
            FunSignature::new(
                &[$($params),* $last],
                Repetition::ZeroOrMore,
                $return_type,
            )
        )
    }};
    ($builtin:ident($($params:ident,)* ...$last:ident) -> $return_type:ident) => {{
        (
            $builtin,
            FunSignature::new(
                &[$($params),* $last],
                Repetition::OneOrMore,
                $return_type,
            )
        )
    }};
}

pub const POW_SIGNATURES: [(BuiltinFunCall, FunSignature); 2] = fun_signatures! {
    PowInt(Int, Int) -> Int
    PowFloat(Float, Float) -> Float
};
pub const LN_SIGNATURES: [(BuiltinFunCall, FunSignature); 1] = fun_signatures! {
    Ln(Float) -> Float
};
pub const SQRT_SIGNATURES: [(BuiltinFunCall, FunSignature); 1] = fun_signatures! {
    Sqrt(Float) -> Float
};
pub const LOG_SIGNATURES: [(BuiltinFunCall, FunSignature); 1] = fun_signatures! {
    Log(Float, Float) -> Float
};
pub const NCR_SIGNATURES: [(BuiltinFunCall, FunSignature); 1] = fun_signatures! {
    Ncr(Int, Int) -> Int
};
pub const TO_DEG_SIGNATURES: [(BuiltinFunCall, FunSignature); 1] = fun_signatures! {
    ToDeg(Float) -> Float
};
pub const TO_RAD_SIGNATURES: [(BuiltinFunCall, FunSignature); 1] = fun_signatures! {
    ToRad(Float) -> Float
};
pub const SIN_SIGNATURES: [(BuiltinFunCall, FunSignature); 1] = fun_signatures! {
    Sin(Float) -> Float
};
pub const COS_SIGNATURES: [(BuiltinFunCall, FunSignature); 1] = fun_signatures! {
    Cos(Float) -> Float
};
pub const TAN_SIGNATURES: [(BuiltinFunCall, FunSignature); 1] = fun_signatures! {
    Tan(Float) -> Float
};
pub const ASIN_SIGNATURES: [(BuiltinFunCall, FunSignature); 1] = fun_signatures! {
    Asin(Float) -> Float
};
pub const ACOS_SIGNATURES: [(BuiltinFunCall, FunSignature); 1] = fun_signatures! {
    Acos(Float) -> Float
};
pub const ATAN_SIGNATURES: [(BuiltinFunCall, FunSignature); 1] = fun_signatures! {
    Atan(Float) -> Float
};
pub const GCD_SIGNATURES: [(BuiltinFunCall, FunSignature); 1] = fun_signatures! {
    Gcd(Int, Int) -> Int
};
pub const MIN_SIGNATURES: [(BuiltinFunCall, FunSignature); 2] = fun_signatures! {
    MinInt(...Int) -> Int
    MinFloat(...Float) -> Int
};
pub const MAX_SIGNATURES: [(BuiltinFunCall, FunSignature); 2] = fun_signatures! {
    MaxInt(...Int) -> Int
    MaxFloat(...Float) -> Int
};
pub const CLAMP_SIGNATURES: [(BuiltinFunCall, FunSignature); 2] = fun_signatures! {
    ClampInt(Int, Int, Int) -> Int
    ClampFloat(Float, Float, Float) -> Float
};
pub const ABS_SIGNATURES: [(BuiltinFunCall, FunSignature); 2] = fun_signatures! {
    AbsInt(Int) -> Int
    AbsFloat(Float) -> Float
};
pub const PRINT_SIGNATURES: [(BuiltinFunCall, FunSignature); 1] = fun_signatures! {
    Print(..Any) -> Unit
};
pub const PRINTLN_SIGNATURES: [(BuiltinFunCall, FunSignature); 1] = fun_signatures! {
    Println(..Any) -> Unit
};
pub const ASSERT_SIGNATURES: [(BuiltinFunCall, FunSignature); 1] = fun_signatures! {
    Assert(Bool) -> Unit
};
pub const ASSERT_EQ_SIGNATURES: [(BuiltinFunCall, FunSignature); 6] = fun_signatures! {
    AssertEq(Int, Int) -> Unit
    AssertEq(Float, Float) -> Unit
    AssertEq(Bool, Bool) -> Unit
    AssertEq(Str, Str) -> Unit
    AssertEq(Range, Range) -> Unit
    AssertEq(Unit, Unit) -> Unit
};
