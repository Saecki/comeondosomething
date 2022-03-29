use std::fmt::Display;
use std::ops::Deref;

use crate::{Context, Expr, ExprT, Range, Val};

impl Context {
    pub fn to_val<'a>(&'a self, expr: &'a Expr) -> crate::Result<&'a Val> {
        match &expr.typ {
            ExprT::Val(p) => Ok(p),
            ExprT::Ident(id) => match self.var_val(*id) {
                Some(d) => Ok(d),
                None => {
                    let name = self.ident_name(*id);
                    Err(crate::Error::UndefinedVar(name.to_owned(), expr.range))
                }
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Return {
    Val(ValRange),
    Unit(Range),
}

impl Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Val(v) => write!(f, "{v}"),
            Self::Unit(_) => write!(f, "()"),
        }
    }
}

impl Return {
    pub fn range(&self) -> Range {
        match self {
            Self::Val(v) => v.range,
            Self::Unit(r) => *r,
        }
    }

    pub fn to_val(&self) -> crate::Result<&ValRange> {
        match self {
            Self::Val(v) => Ok(v),
            Self::Unit(r) => Err(crate::Error::ExpectedValue(*r)),
        }
    }

    pub fn into_val(self) -> crate::Result<ValRange> {
        match self {
            Self::Val(v) => Ok(v),
            Self::Unit(r) => Err(crate::Error::ExpectedValue(r)),
        }
    }

    pub fn to_int(&self) -> crate::Result<i128> {
        self.to_val()?.to_int()
    }

    pub fn to_f64(&self) -> crate::Result<f64> {
        self.to_val()?.to_f64()
    }

    pub fn to_bool(&self) -> crate::Result<bool> {
        self.to_val()?.to_bool()
    }

    pub fn to_str(&self) -> crate::Result<&str> {
        self.to_val()?.to_str()
    }

    pub fn into_str(self) -> crate::Result<String> {
        self.into_val()?.into_str()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ValRange {
    pub val: Val,
    pub range: Range,
}

impl Deref for ValRange {
    type Target = Val;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl std::ops::DerefMut for ValRange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.val
    }
}

impl Display for ValRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl ValRange {
    pub const fn new(val: Val, range: Range) -> Self {
        Self { val, range }
    }

    pub fn to_int(&self) -> crate::Result<i128> {
        self.val
            .to_int()
            .ok_or_else(|| crate::Error::ExpectedInt(self.clone()))
    }

    pub fn to_f64(&self) -> crate::Result<f64> {
        self.val
            .to_f64()
            .ok_or_else(|| crate::Error::ExpectedNumber(self.clone()))
    }

    pub fn to_bool(&self) -> crate::Result<bool> {
        self.val
            .to_bool()
            .ok_or_else(|| crate::Error::ExpectedBool(self.clone()))
    }

    pub fn to_str(&self) -> crate::Result<&str> {
        self.val
            .to_str()
            .ok_or_else(|| crate::Error::ExpectedStr(self.clone()))
    }

    pub fn into_str(self) -> crate::Result<String> {
        match self.val {
            Val::Str(s) => Ok(s),
            Val::Int(_) | Val::Float(_) | Val::Bool(_) => Err(crate::Error::ExpectedStr(self)),
        }
    }
}

impl Val {
    pub fn convert_to_int(&self) -> Option<i128> {
        match self {
            Self::Int(i) => Some(*i),
            Self::Float(f) => {
                let i = *f as i128;
                #[allow(clippy::float_cmp)]
                if i as f64 == *f {
                    Some(i)
                } else {
                    None
                }
            }
            Self::Bool(_) | Self::Str(_) => None,
        }
    }

    pub fn to_int(&self) -> Option<i128> {
        match self {
            Self::Int(i) => Some(*i),
            Self::Float(_) | Self::Bool(_) | Self::Str(_) => None,
        }
    }

    pub fn to_f64(&self) -> Option<f64> {
        match self {
            Self::Int(i) => Some(*i as f64),
            Self::Float(f) => Some(*f),
            Self::Bool(_) | Self::Str(_) => None,
        }
    }

    pub fn to_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            Self::Int(_) | Self::Float(_) | Self::Str(_) => None,
        }
    }

    pub fn to_str(&self) -> Option<&str> {
        match self {
            Self::Str(s) => Some(s),
            Self::Int(_) | Self::Float(_) | Self::Bool(_) => None,
        }
    }

    pub fn into_str(self) -> Option<String> {
        match self {
            Self::Str(s) => Some(s),
            Self::Int(_) | Self::Float(_) | Self::Bool(_) => None,
        }
    }
}
