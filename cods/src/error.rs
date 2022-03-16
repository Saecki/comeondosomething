use std::error;
use std::fmt::{self, Debug, Display};

use crate::{Fun, Sep, SepT, Sign, ValRange};
use crate::{Op, Par, Range};

pub type Result<T> = std::result::Result<T, Error>;

pub trait UserFacing: Sized + Debug + Display {
    fn ranges(&self) -> Vec<Range>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    MissingExpr,
    ExpectedValue(Range),
    ExpectedNumber(ValRange),
    ExpectedBool(ValRange),
    Parsing(Range),
    MissingOperand(Range),
    MissingOperator(Range),
    MissingClosingParenthesis(Par),
    MissingFunctionParentheses(Range),
    MissingFunctionArguments {
        range: Range,
        expected: usize,
        found: usize,
    },
    UnexpectedFunctionArguments {
        ranges: Vec<Range>,
        expected: usize,
        found: usize,
    },
    UnexpectedOperator(Op),
    UnexpectedSeparator(Sep),
    UnexpectedParenthesis(Par),
    InvalidChar(Range),
    UndefinedVar(String, Range),
    InvalidNumberFormat(Range),
    AddOverflow(ValRange, ValRange),
    SubOverflow(ValRange, ValRange),
    MulOverflow(ValRange, ValRange),
    PowOverflow(ValRange, ValRange),
    DivideByZero(ValRange, ValRange),
    FractionEuclidDiv(ValRange, ValRange),
    RemainderByZero(ValRange, ValRange),
    FractionRemainder(ValRange, ValRange),
    FractionGcd(ValRange, ValRange),
    NegativeNcr(ValRange),
    InvalidNcr(ValRange, ValRange),
    FractionNcr(ValRange, ValRange),
    FactorialOverflow(ValRange),
    NegativeFactorial(ValRange),
    FractionFactorial(ValRange),
    InvalidClampBounds(ValRange, ValRange),
    InvalidBwOr(ValRange, ValRange),
    InvalidBwAnd(ValRange, ValRange),
    InvalidAssignment(Range, Range),
    AssertFailed(Range),
    AssertEqFailed(ValRange, ValRange),
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingExpr => write!(f, "Missing expression"),
            Self::ExpectedValue(_) => write!(f, "Expected a value found unit"),
            Self::ExpectedNumber(v) => {
                write!(f, "Expected a number found '{v}' of type {}", v.type_name())
            }
            Self::ExpectedBool(v) => {
                write!(f, "Expected a bool found '{v}' of type {}", v.type_name())
            }
            Self::Parsing(_) => write!(f, "A parsing error occured"),
            Self::MissingOperand(_) => write!(f, "Missing operand"),
            Self::MissingOperator(_) => write!(f, "Missing operator"),
            Self::MissingFunctionParentheses(_) => write!(f, "Missing function call parentheses"),
            Self::MissingClosingParenthesis(_) => write!(f, "Missing closing parenthesis"),
            Self::MissingFunctionArguments {
                expected, found, ..
            } => {
                let missing = expected - found;
                let arg_s = if missing == 1 { "" } else { "s" };
                let are_is = if *expected == 1 { "is" } else { "are" };
                let were_was = if *found == 1 { "was" } else { "were" };
                write!(f, "Missing {missing} function argument{arg_s}, {expected} {are_is} required, but only {found} {were_was} found")
            }
            Self::UnexpectedFunctionArguments {
                expected, found, ..
            } => {
                let over = found - expected;
                let arg_s = if over == 1 { "" } else { "s" };
                let are_is = if *expected == 1 { "is" } else { "are" };
                let were_was = if *found == 1 { "was" } else { "were" };
                write!(f, "Found {over} unexpected function argument{arg_s}, only {expected} {are_is} required, but {found} {were_was} found")
            }
            Self::UnexpectedOperator(_) => write!(f, "Unexpected operator"),
            Self::UnexpectedSeparator(_) => write!(f, "Unexpected separator"),
            Self::UnexpectedParenthesis(_) => write!(f, "Unexpected parenthesis"),
            Self::InvalidChar(_) => write!(f, "Unknown value"),
            Self::UndefinedVar(name, _) => write!(f, "Undefined variable '{name}'"),
            Self::InvalidNumberFormat(_) => write!(f, "Invalid number format"),
            Self::AddOverflow(_, _) => write!(f, "Addition would overflow"),
            Self::SubOverflow(_, _) => write!(f, "Subtraction would overflow"),
            Self::MulOverflow(_, _) => write!(f, "Multiplication would overflow"),
            Self::PowOverflow(_, _) => write!(f, "Exponentiation would overflow"),
            Self::DivideByZero(_, _) => write!(f, "Attempted to divide by 0"),
            Self::FractionEuclidDiv(_, _) => write!(f, "Attempted divide fractions with remainder"),
            Self::RemainderByZero(_, _) => {
                write!(
                    f,
                    "Attempted to calculate the remainder with a divisor of 0"
                )
            }
            Self::FractionRemainder(_, _) => {
                write!(
                    f,
                    "Attempted to calculate the remainder of a division of fractions"
                )
            }
            Self::FractionGcd(_, _) => {
                write!(
                    f,
                    "Attempted to calculate the greatest common divisor of fractions"
                )
            }
            Self::FractionNcr(_, _) => {
                write!(
                    f,
                    "Attempted to calculate the binomial coefficent of fractions"
                )
            }
            Self::NegativeNcr(r) => {
                write!(
                    f,
                    "Attempted to calculate the binomial coefficent with a negative value for r: '{r}'"
                )
            }
            Self::InvalidNcr(n, r) => {
                write!(
                    f,
                    "Attempted to calculate the binomial coefficent with n: '{n}' less than r: '{r}'"
                )
            }
            Self::NegativeFactorial(_) => {
                write!(
                    f,
                    "Attempted to calculate the factorial of a negative number"
                )
            }
            Self::FactorialOverflow(v) => write!(f, "Factorial of '{v}' would overflow"),
            Self::FractionFactorial(_) => {
                write!(f, "Attempted to calculate the factorial of a fraction")
            }
            Self::InvalidClampBounds(min, max) => {
                write!(
                    f,
                    "Invalid clamp bounds min: '{min}' is greater than max: '{max}'"
                )
            }
            Self::InvalidBwOr(a, b) => {
                write!(
                    f,
                    "A bitwise or can only be applied to two ints or two bools, not '{a}' of type {} and '{b}' of type {}",
                    a.type_name(),
                    b.type_name(),
                )
            }
            Self::InvalidBwAnd(a, b) => {
                write!(
                    f,
                    "A bitwise and can only be applied to two ints or two bools, not '{a}' of type {} and '{b}' of type {}",
                    a.type_name(),
                    b.type_name(),
                )
            }
            Self::InvalidAssignment(_, _) => {
                write!(f, "Cannot assign to something that is not a variable")
            }
            Self::AssertFailed(_) => {
                write!(f, "Assertion failed")
            }
            Self::AssertEqFailed(a, b) => {
                write!(f, "Assertion failed: '{a}' == '{b}'")
            }
        }
    }
}

impl UserFacing for Error {
    fn ranges(&self) -> Vec<Range> {
        match self {
            Self::MissingExpr => vec![],
            Self::ExpectedValue(r) => vec![*r],
            Self::ExpectedNumber(v) => vec![v.range],
            Self::ExpectedBool(v) => vec![v.range],
            Self::Parsing(r) => vec![*r],
            Self::MissingOperand(r) => vec![*r],
            Self::MissingOperator(r) => vec![*r],
            Self::MissingFunctionParentheses(r) => vec![*r],
            Self::MissingClosingParenthesis(p) => vec![p.range],
            Self::MissingFunctionArguments { range: pos, .. } => vec![*pos],
            Self::UnexpectedFunctionArguments { ranges, .. } => ranges.clone(),
            Self::UnexpectedOperator(o) => vec![o.range],
            Self::UnexpectedSeparator(s) => vec![s.range],
            Self::UnexpectedParenthesis(p) => vec![p.range],
            Self::InvalidChar(r) => vec![*r],
            Self::UndefinedVar(_, r) => vec![*r],
            Self::InvalidNumberFormat(r) => vec![*r],
            Self::AddOverflow(a, b) => vec![a.range, b.range],
            Self::SubOverflow(a, b) => vec![a.range, b.range],
            Self::MulOverflow(a, b) => vec![a.range, b.range],
            Self::PowOverflow(a, b) => vec![a.range, b.range],
            Self::DivideByZero(a, b) => vec![a.range, b.range],
            Self::FractionEuclidDiv(a, b) => vec![a.range, b.range],
            Self::RemainderByZero(a, b) => vec![a.range, b.range],
            Self::FractionRemainder(a, b) => vec![a.range, b.range],
            Self::FractionGcd(a, b) => vec![a.range, b.range],
            Self::FractionNcr(a, b) => vec![a.range, b.range],
            Self::NegativeNcr(a) => vec![a.range],
            Self::InvalidNcr(a, b) => vec![a.range, b.range],
            Self::FactorialOverflow(v) => vec![v.range],
            Self::NegativeFactorial(v) => vec![v.range],
            Self::FractionFactorial(v) => vec![v.range],
            Self::InvalidClampBounds(min, max) => vec![min.range, max.range],
            Self::InvalidBwOr(a, b) => vec![a.range, b.range],
            Self::InvalidBwAnd(a, b) => vec![a.range, b.range],
            Self::InvalidAssignment(a, b) => vec![*a, *b],
            Self::AssertFailed(r) => vec![*r],
            Self::AssertEqFailed(a, b) => vec![a.range, b.range],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Warning {
    ConfusingCase(Range, &'static str),
    SignFollowingAddition(Range, Range, Sign, usize),
    SignFollowingSubtraction(Range, Range, Sign, usize),
    MultipleSigns(Range, Sign),
    MismatchedParentheses(Par, Par),
    ConfusingFunctionParentheses {
        fun: Fun,
        open_par: Range,
        close_par: Range,
    },
    ConfusingSeparator {
        sep: Sep,
        expected: SepT,
    },
}

impl Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConfusingCase(_, lit) => write!(f, "Confusing casing, consider writing '{lit}'"),
            Self::SignFollowingAddition(_, _, s, c) => {
                let sign_s = if *c == 1 { "" } else { "s" };
                let suggestion = if s.is_positive() {
                    "consider removing them"
                } else {
                    "consider making this a subtraction"
                };
                write!(f, "Sign{sign_s} following an addition, {suggestion}")
            }
            Self::SignFollowingSubtraction(_, _, s, c) => {
                let sign_s = if *c == 1 { "" } else { "s" };
                let suggestion = if s.is_positive() {
                    "consider making this an addition"
                } else {
                    "consider removing them"
                };
                write!(f, "Sign{sign_s} following a subtraction, {suggestion}")
            }
            Self::MultipleSigns(_, s) => {
                if s.is_positive() {
                    write!(f, "Multiple consecutive signs canceling each other out, consider removing them")
                } else {
                    write!(
                        f,
                        "Multiple consecutive signs, consider using a single negation"
                    )
                }
            }
            Self::MismatchedParentheses(_, _) => write!(f, "Parentheses do not match"),
            Self::ConfusingFunctionParentheses { .. } => {
                write!(f, "Functions should use round parentheses")
            }
            Self::ConfusingSeparator { sep, expected } => {
                write!(
                    f,
                    "Confusing separator, expected {expected} found {}",
                    sep.typ
                )
            }
        }
    }
}

impl UserFacing for Warning {
    fn ranges(&self) -> Vec<Range> {
        match self {
            Self::ConfusingCase(r, _) => vec![*r],
            Self::SignFollowingAddition(or, sr, s, _) => {
                if s.is_positive() {
                    vec![*sr]
                } else {
                    vec![*or, *sr]
                }
            }
            Self::SignFollowingSubtraction(or, sr, s, _) => {
                if s.is_positive() {
                    vec![*or, *sr]
                } else {
                    vec![*sr]
                }
            }
            Self::MultipleSigns(r, _) => vec![*r],
            Self::MismatchedParentheses(a, b) => vec![a.range, b.range],
            Self::ConfusingFunctionParentheses {
                fun,
                open_par,
                close_par,
            } => vec![fun.range, *open_par, *close_par],
            Self::ConfusingSeparator { sep, .. } => vec![sep.range],
        }
    }
}
