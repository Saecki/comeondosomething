use std::convert::TryFrom;
use std::io::Write;
use std::ops::{Deref, DerefMut};

use crate::{CRange, Context, Expr, ExprT, IdentRange, Range, Val};

pub use scope::*;
pub use val::*;

mod scope;
#[cfg(test)]
mod test;
mod val;

#[derive(Clone, Debug, PartialEq)]
pub struct Ast {
    pub typ: AstT,
    pub range: CRange,
}

impl Deref for Ast {
    type Target = AstT;

    fn deref(&self) -> &Self::Target {
        &self.typ
    }
}

impl DerefMut for Ast {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.typ
    }
}

impl Ast {
    pub fn new(typ: AstT, range: CRange) -> Self {
        Self { typ, range }
    }

    pub fn expr(val: Expr) -> Self {
        let r = val.range;
        Self::new(AstT::Expr(val), r)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AstT {
    Empty,
    Error,
    Expr(Expr),
    Block(Vec<Ast>),
    IfExpr(IfExpr),
    WhileLoop(Box<CondBlock>),
    ForLoop(ForLoop),
    FunDef(IdentRange, Vec<IdentRange>, Block),
    FunCall(IdentRange, Vec<Ast>),
    VarDef(IdentRange, Box<Ast>, bool),
    Assign(IdentRange, Box<Ast>),
    AddAssign(IdentRange, Box<Ast>),
    SubAssign(IdentRange, Box<Ast>),
    MulAssign(IdentRange, Box<Ast>),
    DivAssign(IdentRange, Box<Ast>),
    RangeEx(Box<Ast>, Box<Ast>),
    RangeIn(Box<Ast>, Box<Ast>),
    Neg(Box<Ast>),
    Add(Box<Ast>, Box<Ast>),
    Sub(Box<Ast>, Box<Ast>),
    Mul(Box<Ast>, Box<Ast>),
    Div(Box<Ast>, Box<Ast>),
    IntDiv(Box<Ast>, Box<Ast>),
    Rem(Box<Ast>, Box<Ast>),
    Pow(Box<Ast>, Box<Ast>),
    Eq(Box<Ast>, Box<Ast>),
    Ne(Box<Ast>, Box<Ast>),
    Lt(Box<Ast>, Box<Ast>),
    Le(Box<Ast>, Box<Ast>),
    Gt(Box<Ast>, Box<Ast>),
    Ge(Box<Ast>, Box<Ast>),
    Or(Box<Ast>, Box<Ast>),
    And(Box<Ast>, Box<Ast>),
    BwOr(Box<Ast>, Box<Ast>),
    BwAnd(Box<Ast>, Box<Ast>),
    Not(Box<Ast>),
    Degree(Box<Ast>),
    Radian(Box<Ast>),
    Factorial(Box<Ast>),
    Ln(Box<Ast>),
    Log(Box<Ast>, Box<Ast>),
    Sqrt(Box<Ast>),
    Ncr(Box<Ast>, Box<Ast>),
    Sin(Box<Ast>),
    Cos(Box<Ast>),
    Tan(Box<Ast>),
    Asin(Box<Ast>),
    Acos(Box<Ast>),
    Atan(Box<Ast>),
    Gcd(Box<Ast>, Box<Ast>),
    Min(Vec<Ast>),
    Max(Vec<Ast>),
    Clamp(Box<Ast>, Box<Ast>, Box<Ast>),
    Print(Vec<Ast>),
    Println(Vec<Ast>),
    Spill,
    Assert(Box<Ast>),
    AssertEq(Box<Ast>, Box<Ast>),
}

impl AstT {
    pub fn as_ident(&self) -> Option<IdentRange> {
        match self {
            Self::Expr(e) => e.as_ident(),
            _ => None,
        }
    }
}

impl AstT {
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IfExpr {
    pub cases: Vec<CondBlock>,
    pub else_block: Option<Block>,
}

impl IfExpr {
    pub const fn new(cases: Vec<CondBlock>, else_block: Option<Block>) -> Self {
        Self { cases, else_block }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CondBlock {
    pub cond: Ast,
    pub block: Vec<Ast>,
    pub range: CRange,
}

impl CondBlock {
    pub const fn new(cond: Ast, block: Vec<Ast>, range: CRange) -> Self {
        Self { cond, block, range }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForLoop {
    pub ident: IdentRange,
    pub iter: Box<Ast>,
    pub block: Block,
}

impl ForLoop {
    pub const fn new(ident: IdentRange, iter: Box<Ast>, block: Block) -> Self {
        Self { ident, iter, block }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub asts: Vec<Ast>,
    pub range: CRange,
}

impl Block {
    pub const fn new(asts: Vec<Ast>, range: CRange) -> Self {
        Self { asts, range }
    }
}

impl Context {
    /// Evaluate all ast's and return the last value.
    pub fn eval_all(&mut self, asts: &[Ast]) -> crate::Result<Option<Val>> {
        match asts.split_last() {
            Some((last, others)) => {
                for c in others {
                    self.eval_ast(c)?;
                }
                self.eval(last)
            }
            None => Err(crate::Error::MissingExpr),
        }
    }

    pub fn eval(&mut self, ast: &Ast) -> crate::Result<Option<Val>> {
        match self.eval_ast(ast)? {
            Return::Val(v) => Ok(Some(v.val)),
            Return::Unit(_) => Ok(None),
        }
    }

    pub fn eval_to_vals(&mut self, args: &[Ast]) -> crate::Result<Vec<ValRange>> {
        let mut vals = Vec::with_capacity(args.len());
        for a in args {
            vals.push(self.eval_to_val(a)?);
        }
        Ok(vals)
    }

    pub fn eval_to_val(&mut self, ast: &Ast) -> crate::Result<ValRange> {
        self.eval_ast(ast)?.into_val()
    }

    pub fn eval_to_int(&mut self, ast: &Ast) -> crate::Result<i128> {
        self.eval_ast(ast)?.to_int()
    }

    pub fn eval_to_f64(&mut self, ast: &Ast) -> crate::Result<f64> {
        self.eval_ast(ast)?.to_f64()
    }

    pub fn eval_to_bool(&mut self, ast: &Ast) -> crate::Result<bool> {
        self.eval_ast(ast)?.to_bool()
    }

    pub fn eval_to_range(&mut self, ast: &Ast) -> crate::Result<Range> {
        self.eval_ast(ast)?.to_range()
    }

    pub fn eval_ast(&mut self, ast: &Ast) -> crate::Result<Return> {
        let r = ast.range;
        match &ast.typ {
            AstT::Empty => Ok(Return::Unit(r)),
            AstT::Error => Err(crate::Error::Parsing(r)),
            AstT::Expr(e) => self.expr(e, r),
            AstT::Block(a) => self.block(a, r),
            AstT::IfExpr(a) => self.if_expr(a, r),
            AstT::WhileLoop(a) => self.while_loop(a, r),
            AstT::ForLoop(a) => self.for_loop(a, r),
            AstT::FunDef(a, b, c) => self.fun_def(a, b, c, r),
            AstT::FunCall(a, b) => self.fun_call(a, b, r),
            AstT::VarDef(a, b, c) => self.var_def(a, b, *c, r),
            AstT::Assign(a, b) => self.assign(a, b, r),
            AstT::AddAssign(a, b) => self.add_assign(a, b, r),
            AstT::SubAssign(a, b) => self.sub_assign(a, b, r),
            AstT::MulAssign(a, b) => self.mul_assign(a, b, r),
            AstT::DivAssign(a, b) => self.div_assign(a, b, r),
            AstT::RangeEx(a, b) => self.range_ex(a, b, r),
            AstT::RangeIn(a, b) => self.range_in(a, b, r),
            AstT::Neg(a) => self.neg(a, r),
            AstT::Add(a, b) => self.add(a, b, r),
            AstT::Sub(a, b) => self.sub(a, b, r),
            AstT::Mul(a, b) => self.mul(a, b, r),
            AstT::Div(a, b) => self.div(a, b, r),
            AstT::IntDiv(a, b) => self.int_div(a, b, r),
            AstT::Rem(a, b) => self.rem(a, b, r),
            AstT::Pow(a, b) => self.pow(a, b, r),
            AstT::Eq(a, b) => self.eq(a, b, r),
            AstT::Ne(a, b) => self.ne(a, b, r),
            AstT::Lt(a, b) => self.lt(a, b, r),
            AstT::Le(a, b) => self.le(a, b, r),
            AstT::Gt(a, b) => self.gt(a, b, r),
            AstT::Ge(a, b) => self.ge(a, b, r),
            AstT::Or(a, b) => self.or(a, b, r),
            AstT::And(a, b) => self.and(a, b, r),
            AstT::BwOr(a, b) => self.bw_or(a, b, r),
            AstT::BwAnd(a, b) => self.bw_and(a, b, r),
            AstT::Not(a) => self.not(a, r),
            AstT::Degree(a) => self.degree(a, r),
            AstT::Radian(a) => self.radian(a, r),
            AstT::Factorial(a) => self.factorial(a, r),
            AstT::Ln(a) => self.ln(a, r),
            AstT::Log(a, b) => self.log(a, b, r),
            AstT::Sqrt(a) => self.sqrt(a, r),
            AstT::Ncr(a, b) => self.ncr(a, b, r),
            AstT::Sin(a) => self.sin(a, r),
            AstT::Cos(a) => self.cos(a, r),
            AstT::Tan(a) => self.tan(a, r),
            AstT::Asin(a) => self.asin(a, r),
            AstT::Acos(a) => self.acos(a, r),
            AstT::Atan(a) => self.atan(a, r),
            AstT::Gcd(a, b) => self.gcd(a, b, r),
            AstT::Min(args) => self.min(args, r),
            AstT::Max(args) => self.max(args, r),
            AstT::Clamp(num, min, max) => self.clamp(num, min, max, r),
            AstT::Print(args) => self.print(args, r),
            AstT::Println(args) => self.println(args, r),
            AstT::Spill => self.spill(r),
            AstT::Assert(a) => self.assert(a, r),
            AstT::AssertEq(a, b) => self.assert_eq(a, b, r),
        }
        .map(|mut r| {
            if let Return::Val(v) = &mut r {
                if let Some(i) = v.convert_to_int() {
                    v.val = Val::Int(i);
                }
            }
            r
        })
    }

    fn expr(&mut self, expr: &Expr, range: CRange) -> crate::Result<Return> {
        match &expr.typ {
            ExprT::Val(v) => return_val(v.clone(), range),
            &ExprT::Ident(id) => {
                let ident = IdentRange::new(id, expr.range);
                Ok(Return::Val(self.resolve_var(&ident)?))
            }
        }
    }

    fn block(&mut self, asts: &[Ast], range: CRange) -> crate::Result<Return> {
        self.scopes.push(Scope::default());
        let r = match asts.split_last() {
            Some((last, others)) => {
                for c in others {
                    self.eval_ast(c)?;
                }
                self.eval_ast(last)
            }
            None => Ok(Return::Unit(range)),
        };
        self.scopes.pop();
        r
    }

    fn if_expr(&mut self, if_expr: &IfExpr, range: CRange) -> crate::Result<Return> {
        for c in if_expr.cases.iter() {
            if self.eval_to_bool(&c.cond)? {
                return self.block(&c.block, range);
            }
        }
        if let Some(b) = &if_expr.else_block {
            return self.block(&b.asts, b.range);
        }
        Ok(Return::Unit(range))
    }

    fn while_loop(&mut self, whl_loop: &CondBlock, range: CRange) -> crate::Result<Return> {
        while self.eval_to_bool(&whl_loop.cond)? {
            self.block(&whl_loop.block, range)?;
        }
        Ok(Return::Unit(range))
    }

    fn for_loop(&mut self, for_loop: &ForLoop, range: CRange) -> crate::Result<Return> {
        let iter = self.eval_to_range(&for_loop.iter)?;

        for i in iter.iter() {
            let mut scope = Scope::default();
            scope.def_var(for_loop.ident, Some(Val::Int(i)), false);
            self.scopes.push(scope);
            for c in for_loop.block.asts.iter() {
                self.eval_ast(c)?;
            }
            self.scopes.pop();
        }

        Ok(Return::Unit(range))
    }

    fn fun_def(
        &mut self,
        id: &IdentRange,
        params: &[IdentRange],
        block: &Block,
        range: CRange,
    ) -> crate::Result<Return> {
        self.def_fun(*id, params.to_owned(), block.to_owned())?;
        Ok(Return::Unit(range))
    }

    fn fun_call(&mut self, id: &IdentRange, args: &[Ast], range: CRange) -> crate::Result<Return> {
        // PERF: avoid clone
        let fun = self.resolve_fun(id)?.clone();

        if args.len() < fun.params.len() {
            return Err(crate::Error::MissingFunArgs {
                range: CRange::pos(range.end - 1),
                expected: fun.params.len(),
                found: args.len(),
            });
        }

        if args.len() > fun.params.len() {
            let ranges = args
                .iter()
                .skip(fun.params.len())
                .map(|a| a.range)
                .collect();
            return Err(crate::Error::UnexpectedFunArgs {
                ranges,
                expected: fun.params.len(),
                found: args.len(),
            });
        }

        let mut scope = Scope::default();
        for (p, v) in fun.params.iter().zip(args) {
            let val = self.eval_to_val(v)?;
            scope.def_var(*p, Some(val.val), false);
        }
        self.scopes.push(scope);

        let r = match fun.block.asts.split_last() {
            Some((last, others)) => {
                for c in others {
                    self.eval_ast(c)?;
                }
                self.eval_ast(last)
            }
            None => Ok(Return::Unit(range)),
        };

        self.scopes.pop();

        r
    }

    fn var_def(
        &mut self,
        id: &IdentRange,
        b: &Ast,
        mutable: bool,
        range: CRange,
    ) -> crate::Result<Return> {
        let val = self.eval_to_val(b)?;
        self.def_var(*id, Some(val.val), mutable);
        Ok(Return::Unit(range))
    }

    fn assign(&mut self, id: &IdentRange, b: &Ast, range: CRange) -> crate::Result<Return> {
        let v = self.eval_to_val(b)?;
        self.set_var(id, Some(v.val), v.range)?;
        Ok(Return::Unit(range))
    }

    fn add_assign(&mut self, id: &IdentRange, b: &Ast, range: CRange) -> crate::Result<Return> {
        let va = self.resolve_var(id)?;
        let vb = self.eval_to_val(b)?;
        let val_r = vb.range;
        let val = checked_add(va, vb)?;
        self.set_var(id, Some(val), val_r)?;
        Ok(Return::Unit(range))
    }

    fn sub_assign(&mut self, id: &IdentRange, b: &Ast, range: CRange) -> crate::Result<Return> {
        let va = self.resolve_var(id)?;
        let vb = self.eval_to_val(b)?;
        let val_r = vb.range;
        let val = checked_sub(va, vb)?;
        self.set_var(id, Some(val), val_r)?;
        Ok(Return::Unit(range))
    }

    fn mul_assign(&mut self, id: &IdentRange, b: &Ast, range: CRange) -> crate::Result<Return> {
        let va = self.resolve_var(id)?;
        let vb = self.eval_to_val(b)?;
        let val_r = vb.range;
        let val = checked_mul(va, vb)?;
        self.set_var(id, Some(val), val_r)?;
        Ok(Return::Unit(range))
    }

    fn div_assign(&mut self, id: &IdentRange, b: &Ast, range: CRange) -> crate::Result<Return> {
        let va = self.resolve_var(id)?;
        let vb = self.eval_to_val(b)?;
        let val_r = vb.range;
        let val = checked_div(va, vb)?;
        self.set_var(id, Some(val), val_r)?;
        Ok(Return::Unit(range))
    }

    fn range_ex(&mut self, a: &Ast, b: &Ast, range: CRange) -> crate::Result<Return> {
        let va = self.eval_to_int(a)?;
        let vb = self.eval_to_int(b)?;
        let val = Val::Range(Range::Exclusive(va, vb));
        return_val(val, range)
    }

    fn range_in(&mut self, a: &Ast, b: &Ast, range: CRange) -> crate::Result<Return> {
        let va = self.eval_to_int(a)?;
        let vb = self.eval_to_int(b)?;
        let val = Val::Range(Range::Inclusive(va, vb));
        return_val(val, range)
    }

    fn neg(&mut self, n: &Ast, range: CRange) -> crate::Result<Return> {
        let v = self.eval_to_val(n)?;
        let val = match v.val {
            Val::Int(i) => Val::Int(-i),
            _ => Val::Float(-v.to_f64()?),
        };
        return_val(val, range)
    }

    fn add(&mut self, a: &Ast, b: &Ast, range: CRange) -> crate::Result<Return> {
        let va = self.eval_to_val(a)?;
        let vb = self.eval_to_val(b)?;
        let val = checked_add(va, vb)?;
        return_val(val, range)
    }

    fn sub(&mut self, a: &Ast, b: &Ast, range: CRange) -> crate::Result<Return> {
        let va = self.eval_to_val(a)?;
        let vb = self.eval_to_val(b)?;
        let val = checked_sub(va, vb)?;
        return_val(val, range)
    }

    fn mul(&mut self, n1: &Ast, n2: &Ast, range: CRange) -> crate::Result<Return> {
        let va = self.eval_to_val(n1)?;
        let vb = self.eval_to_val(n2)?;
        let val = checked_mul(va, vb)?;
        return_val(val, range)
    }

    fn div(&mut self, n1: &Ast, n2: &Ast, range: CRange) -> crate::Result<Return> {
        let va = self.eval_to_val(n1)?;
        let vb = self.eval_to_val(n2)?;
        let val = checked_div(va, vb)?;
        return_val(val, range)
    }

    fn int_div(&mut self, n1: &Ast, n2: &Ast, range: CRange) -> crate::Result<Return> {
        let va = self.eval_to_val(n1)?;
        let vb = self.eval_to_val(n2)?;

        let val = match (&va.val, &vb.val) {
            (&Val::Int(a), &Val::Int(b)) => {
                if b == 0 {
                    return Err(crate::Error::DivideByZero(va.clone(), vb.clone()));
                }
                Val::Int(a / b)
            }
            _ => return Err(crate::Error::FractionEuclidDiv(va.clone(), vb.clone())),
        };
        return_val(val, range)
    }

    fn rem(&mut self, n1: &Ast, n2: &Ast, range: CRange) -> crate::Result<Return> {
        let va = self.eval_to_val(n1)?;
        let vb = self.eval_to_val(n2)?;

        let val = match (&va.val, &vb.val) {
            (&Val::Int(a), &Val::Int(b)) => {
                if b == 0 {
                    return Err(crate::Error::RemainderByZero(va.clone(), vb.clone()));
                }

                let r = a % b;
                if (r > 0 && b < 0) || (r < 0 && b > 0) {
                    Val::Int(r + b)
                } else {
                    Val::Int(r)
                }
            }
            _ => return Err(crate::Error::FractionRemainder(va.clone(), vb.clone())),
        };
        return_val(val, range)
    }

    fn pow(&mut self, n1: &Ast, n2: &Ast, range: CRange) -> crate::Result<Return> {
        let va = self.eval_to_val(n1)?;
        let vb = self.eval_to_val(n2)?;

        let val = match (&va.val, &vb.val) {
            (&Val::Int(base), &Val::Int(exp)) => {
                if let Ok(e) = u32::try_from(exp) {
                    Val::Int(base.pow(e))
                } else if let Ok(e) = i32::try_from(exp) {
                    Val::Float((base as f64).powi(e))
                } else {
                    return Err(crate::Error::PowOverflow(va.clone(), vb.clone()));
                }
            }
            _ => Val::Float(va.to_f64()?.powf(vb.to_f64()?)),
        };
        return_val(val, range)
    }

    fn eq(&mut self, a: &Ast, b: &Ast, range: CRange) -> crate::Result<Return> {
        let a = self.eval_to_val(a)?;
        let b = self.eval_to_val(b)?;

        return_val(Val::Bool(a.val == b.val), range)
    }

    fn ne(&mut self, a: &Ast, b: &Ast, range: CRange) -> crate::Result<Return> {
        let a = self.eval_to_val(a)?;
        let b = self.eval_to_val(b)?;

        return_val(Val::Bool(a.val != b.val), range)
    }

    fn lt(&mut self, a: &Ast, b: &Ast, range: CRange) -> crate::Result<Return> {
        let va = self.eval_to_f64(a)?;
        let vb = self.eval_to_f64(b)?;

        return_val(Val::Bool(va < vb), range)
    }

    fn le(&mut self, a: &Ast, b: &Ast, range: CRange) -> crate::Result<Return> {
        let va = self.eval_to_f64(a)?;
        let vb = self.eval_to_f64(b)?;

        return_val(Val::Bool(va <= vb), range)
    }

    fn gt(&mut self, a: &Ast, b: &Ast, range: CRange) -> crate::Result<Return> {
        let va = self.eval_to_f64(a)?;
        let vb = self.eval_to_f64(b)?;

        return_val(Val::Bool(va > vb), range)
    }

    fn ge(&mut self, a: &Ast, b: &Ast, range: CRange) -> crate::Result<Return> {
        let va = self.eval_to_f64(a)?;
        let vb = self.eval_to_f64(b)?;

        return_val(Val::Bool(va >= vb), range)
    }

    fn or(&mut self, a: &Ast, b: &Ast, range: CRange) -> crate::Result<Return> {
        let a = self.eval_to_bool(a)?;
        let b = self.eval_to_bool(b)?;

        return_val(Val::Bool(a || b), range)
    }

    fn and(&mut self, a: &Ast, b: &Ast, range: CRange) -> crate::Result<Return> {
        let a = self.eval_to_bool(a)?;
        let b = self.eval_to_bool(b)?;

        return_val(Val::Bool(a && b), range)
    }

    fn bw_or(&mut self, a: &Ast, b: &Ast, range: CRange) -> crate::Result<Return> {
        let va = self.eval_to_val(a)?;
        let vb = self.eval_to_val(b)?;

        let val = match (&va.val, &vb.val) {
            (Val::Int(a), Val::Int(b)) => Val::Int(a | b),
            (Val::Bool(a), Val::Bool(b)) => Val::Bool(a | b),
            _ => return Err(crate::Error::InvalidBwOr(va, vb)),
        };

        return_val(val, range)
    }

    fn bw_and(&mut self, a: &Ast, b: &Ast, range: CRange) -> crate::Result<Return> {
        let va = self.eval_to_val(a)?;
        let vb = self.eval_to_val(b)?;

        let val = match (&va.val, &vb.val) {
            (Val::Int(a), Val::Int(b)) => Val::Int(a & b),
            (Val::Bool(a), Val::Bool(b)) => Val::Bool(a & b),
            _ => return Err(crate::Error::InvalidBwAnd(va, vb)),
        };

        return_val(val, range)
    }

    fn not(&mut self, a: &Ast, range: CRange) -> crate::Result<Return> {
        let va = self.eval_to_bool(a)?;
        return_val(Val::Bool(!va), range)
    }

    // TODO add a angle value type as input for trigeometrical functions
    fn degree(&mut self, n: &Ast, range: CRange) -> crate::Result<Return> {
        let rad = self.eval_to_f64(n)?.to_radians();
        return_val(Val::Float(rad), range)
    }

    fn radian(&mut self, n: &Ast, range: CRange) -> crate::Result<Return> {
        let rad = self.eval_to_f64(n)?;
        return_val(Val::Float(rad), range)
    }

    fn factorial(&mut self, n: &Ast, range: CRange) -> crate::Result<Return> {
        let v = self.eval_to_val(n)?;
        match v.val {
            Val::Int(i) => {
                if i < 0 {
                    Err(crate::Error::NegativeFactorial(v))
                } else {
                    let mut f: i128 = 1;
                    for i in 1..=i {
                        match f.checked_mul(i) {
                            Some(v) => f = v,
                            None => return Err(crate::Error::FactorialOverflow(v)),
                        }
                    }

                    return_val(Val::Int(f), range)
                }
            }
            _ => Err(crate::Error::FractionFactorial(v)),
        }
    }

    fn ln(&mut self, n: &Ast, range: CRange) -> crate::Result<Return> {
        let val = self.eval_to_f64(n)?.ln();
        return_val(Val::Float(val), range)
    }

    fn log(&mut self, base: &Ast, num: &Ast, range: CRange) -> crate::Result<Return> {
        let b = self.eval_to_f64(base)?;
        let n = self.eval_to_f64(num)?;
        let val = Val::Float(n.log(b));
        return_val(val, range)
    }

    fn sqrt(&mut self, n: &Ast, range: CRange) -> crate::Result<Return> {
        let val = self.eval_to_f64(n)?.sqrt();
        return_val(Val::Float(val), range)
    }

    fn ncr(&mut self, n1: &Ast, n2: &Ast, range: CRange) -> crate::Result<Return> {
        let va = self.eval_to_val(n1)?;
        let vb = self.eval_to_val(n2)?;
        let val = match (&va.val, &vb.val) {
            (&Val::Int(n), &Val::Int(mut r)) => {
                if r < 0 {
                    return Err(crate::Error::NegativeNcr(vb));
                }
                if n < r {
                    return Err(crate::Error::InvalidNcr(va, vb));
                }

                // symmetrical: nCr(9, 2) == nCr(9, 7)
                if r > n - r {
                    r = n - r;
                }

                let mut val = 1;
                for i in 1..=r {
                    val *= n - r + i;
                    val /= i;
                }

                Val::Int(val)
            }
            _ => return Err(crate::Error::FractionNcr(va, vb)),
        };
        return_val(val, range)
    }

    fn sin(&mut self, n: &Ast, range: CRange) -> crate::Result<Return> {
        let a = self.eval_to_f64(n)?.sin();
        return_val(Val::Float(a), range)
    }

    fn cos(&mut self, n: &Ast, range: CRange) -> crate::Result<Return> {
        let a = self.eval_to_f64(n)?.cos();
        return_val(Val::Float(a), range)
    }

    fn tan(&mut self, n: &Ast, range: CRange) -> crate::Result<Return> {
        let a = self.eval_to_f64(n)?.tan();
        return_val(Val::Float(a), range)
    }

    fn asin(&mut self, n: &Ast, range: CRange) -> crate::Result<Return> {
        let a = self.eval_to_f64(n)?.asin();
        return_val(Val::Float(a), range)
    }

    fn acos(&mut self, n: &Ast, range: CRange) -> crate::Result<Return> {
        let a = self.eval_to_f64(n)?.acos();
        return_val(Val::Float(a), range)
    }

    fn atan(&mut self, n: &Ast, range: CRange) -> crate::Result<Return> {
        let a = self.eval_to_f64(n)?.atan();
        return_val(Val::Float(a), range)
    }

    fn gcd(&mut self, n1: &Ast, n2: &Ast, range: CRange) -> crate::Result<Return> {
        let va = self.eval_to_val(n1)?;
        let vb = self.eval_to_val(n2)?;
        match (&va.val, &vb.val) {
            (Val::Int(mut a), Val::Int(mut b)) => {
                let mut _t = 0;
                while b != 0 {
                    _t = b;
                    b = a % b;
                    a = _t;
                }
                return_val(Val::Int(a), range)
            }
            _ => Err(crate::Error::FractionGcd(va, vb)),
        }
    }

    fn min(&mut self, args: &[Ast], range: CRange) -> crate::Result<Return> {
        let mut min = None;
        for a in args {
            let val = self.eval_to_val(a)?.to_f64()?;
            match min {
                None => min = Some(val),
                Some(m) => {
                    if val < m {
                        min = Some(val);
                    }
                }
            }
        }

        let max = min.expect("Iterator should at least contain 1 element");
        return_val(Val::Float(max), range)
    }

    fn max(&mut self, args: &[Ast], range: CRange) -> crate::Result<Return> {
        let mut max = None;
        for a in args {
            let val = self.eval_to_f64(a)?;
            match max {
                None => max = Some(val),
                Some(m) => {
                    if val > m {
                        max = Some(val);
                    }
                }
            }
        }

        let max = max.expect("Iterator should at least contain 1 element");
        return_val(Val::Float(max), range)
    }

    fn clamp(&mut self, num: &Ast, min: &Ast, max: &Ast, range: CRange) -> crate::Result<Return> {
        let vnum = self.eval_to_val(num)?;
        let vmin = self.eval_to_val(min)?;
        let vmax = self.eval_to_val(max)?;

        let val = match (&vnum.val, &vmin.val, &vmax.val) {
            (&Val::Int(num), &Val::Int(min), &Val::Int(max)) => {
                if min > max {
                    return Err(crate::Error::InvalidClampBounds(vmin, vmax));
                }
                Val::Int(num.clamp(min, max))
            }
            _ => {
                let num = vnum.to_f64()?;
                let min = vmin.to_f64()?;
                let max = vmax.to_f64()?;
                // floating point weirdness, negated assertion of stdlib
                #[allow(clippy::neg_cmp_op_on_partial_ord)]
                if !(min <= max) {
                    return Err(crate::Error::InvalidClampBounds(vmin, vmax));
                }
                Val::Float(num.clamp(min, max))
            }
        };
        return_val(val, range)
    }

    fn print(&mut self, args: &[Ast], range: CRange) -> crate::Result<Return> {
        let vals = self.eval_to_vals(args)?;
        if let Some((first, others)) = vals.split_first() {
            self.stdio.print(format_args!("{first}"));
            for v in others {
                self.stdio.print(format_args!(" {v}"));
            }
        }
        Ok(Return::Unit(range))
    }

    fn println(&mut self, args: &[Ast], range: CRange) -> crate::Result<Return> {
        self.print(args, range)?;
        let _ = self.stdio.stdout.write_all(&*b"\n");
        Ok(Return::Unit(range))
    }

    fn spill(&mut self, range: CRange) -> crate::Result<Return> {
        for s in self.scopes.iter() {
            for (id, var) in s.vars.iter() {
                if let Some(val) = &var.value {
                    let name = self.idents.name(*id);
                    self.stdio.print(format_args!("{name} = {val}\n"));
                }
            }
        }
        Ok(Return::Unit(range))
    }

    fn assert(&mut self, a: &Ast, range: CRange) -> crate::Result<Return> {
        let va = self.eval_to_bool(a)?;

        if !va {
            return Err(crate::Error::AssertFailed(a.range));
        }

        Ok(Return::Unit(range))
    }

    fn assert_eq(&mut self, a: &Ast, b: &Ast, range: CRange) -> crate::Result<Return> {
        let va = self.eval_to_val(a)?;
        let vb = self.eval_to_val(b)?;

        if va.val != vb.val {
            return Err(crate::Error::AssertEqFailed(va, vb));
        }

        Ok(Return::Unit(range))
    }
}

fn checked_add(va: ValRange, vb: ValRange) -> crate::Result<Val> {
    let val = match (&va.val, &vb.val) {
        (Val::Int(a), &Val::Int(b)) => match a.checked_add(b) {
            Some(v) => Val::Int(v),
            None => return Err(crate::Error::AddOverflow(va, vb)),
        },
        _ => Val::Float(va.to_f64()? + vb.to_f64()?),
    };
    Ok(val)
}

fn checked_sub(va: ValRange, vb: ValRange) -> crate::Result<Val> {
    match (&va.val, &vb.val) {
        (Val::Int(a), &Val::Int(b)) => match a.checked_sub(b) {
            Some(v) => Ok(Val::Int(v)),
            None => Err(crate::Error::SubOverflow(va, vb)),
        },
        _ => Ok(Val::Float(va.to_f64()? - vb.to_f64()?)),
    }
}

fn checked_mul(va: ValRange, vb: ValRange) -> crate::Result<Val> {
    match (&va.val, &vb.val) {
        (Val::Int(a), &Val::Int(b)) => match a.checked_mul(b) {
            Some(v) => Ok(Val::Int(v)),
            None => Err(crate::Error::MulOverflow(va, vb)),
        },
        _ => Ok(Val::Float(va.to_f64()? * vb.to_f64()?)),
    }
}

fn checked_div(va: ValRange, vb: ValRange) -> crate::Result<Val> {
    match (&va.val, &vb.val) {
        (&Val::Int(a), &Val::Int(b)) => {
            if b == 0 {
                Err(crate::Error::DivideByZero(va, vb))
            } else if a % b == 0 {
                Ok(Val::Int(a / b))
            } else {
                Ok(Val::Float(a as f64 / b as f64))
            }
        }
        _ => {
            let divisor = vb.to_f64()?;
            if divisor == 0.0 {
                Err(crate::Error::DivideByZero(va, vb))
            } else {
                Ok(Val::Float(va.to_f64()? / divisor))
            }
        }
    }
}

fn return_val(val: Val, range: CRange) -> crate::Result<Return> {
    Ok(Return::Val(ValRange::new(val, range)))
}
