use cods::{Error, Par, ParT, Range, Val, ValRange};

fn assert(input: &str, expected: Val) {
    match cods::eval(input) {
        Ok(Some(val)) => assert_eq!(val, expected),
        Ok(None) => panic!("Expected a value found nothing"),
        Err(e) => {
            panic!("{e:?}");
        }
    }
}

fn assert_unit(input: &str) {
    match cods::eval(input) {
        Ok(Some(val)) => panic!("Expected unit found value: '{val}'"),
        Ok(None) => (),
        Err(e) => {
            panic!("{e:?}");
        }
    }
}

fn assert_err(input: &str, expected: Error) {
    match cods::eval(input) {
        Ok(_) => panic!("Expected error: {expected:?}"),
        Err(e) => assert_eq!(e, expected),
    }
}

#[test]
fn neg() {
    assert("-32", Val::Int(-32));
    assert("-5.3", Val::Float(-5.3));
}

#[test]
fn float() {
    assert(
        "234.4234 + 6345.423 * 3264.2462",
        Val::Float(20713257.3385426),
    );
}

#[test]
fn int() {
    assert(
        "6 + 3452 − (3252 × 5324) + (((2342 × 3242) ÷ 4234) × 4234) − 324",
        Val::Int(-9717750),
    );
}

#[test]
fn unicode_ops() {
    assert(
        "(23423 × 423 + (423 − 234) ÷ 654 + 4324) × 4234",
        Val::Float(41968480425.587155963),
    );
}

#[test]
fn signs() {
    assert("1 - 2 * -2", Val::Int(5));
}

#[test]
fn euclid_div() {
    assert("8 div 3", Val::Int(2));
}

#[test]
fn remainder() {
    assert("8 % 3", Val::Int(2));
}

#[test]
fn negative_remainder() {
    assert("-8 % 3", Val::Int(1));
    assert("8 % -5", Val::Int(-2));
}

#[test]
fn gcd() {
    assert("gcd(6, 9)", Val::Int(3));
    assert("gcd(4, 0)", Val::Int(4));
    assert("gcd(0, 5)", Val::Int(5));
}

#[test]
fn factorial() {
    assert("8!", Val::Int(8 * 7 * 6 * 5 * 4 * 3 * 2 * 1));
}

#[test]
fn factorial_overflow() {
    assert_err(
        "34!",
        Error::FactorialOverflow(ValRange::new(Val::Int(34), Range::of(0, 2))),
    );
}

#[test]
fn factorial_fraction() {
    assert_err(
        "4.1!",
        Error::FractionFactorial(ValRange::new(Val::Float(4.1), Range::of(0, 3))),
    );
}

#[test]
fn factorial_negative() {
    assert_err(
        "(-3)!",
        Error::NegativeFactorial(ValRange::new(Val::Int(-3), Range::of(0, 4))),
    );
}

#[test]
fn squareroot() {
    assert("sqrt(625)", Val::Int(25));
}

#[test]
fn binomial_coefficient() {
    assert("nCr(6, 2)", Val::Int(15));
    assert("nCr(23, 0)", Val::Int(1));
}

#[test]
fn binomial_coefficient_invalid() {
    assert_err(
        "nCr(3, 4)",
        Error::InvalidNcr(
            ValRange::new(Val::Int(3), Range::pos(4)),
            ValRange::new(Val::Int(4), Range::pos(7)),
        ),
    );
}

#[test]
fn binomial_coefficient_negative() {
    assert_err(
        "nCr(5, -3)",
        Error::NegativeNcr(ValRange::new(Val::Int(-3), Range::of(7, 9))),
    );
}

#[test]
fn ln() {
    assert("ln(e^27)", Val::Int(27));
}

#[test]
fn log2() {
    assert("log(2, 8)", Val::Int(3));
}

#[test]
fn log10() {
    assert("log(10, 100000)", Val::Int(5));
}

#[test]
fn min() {
    assert("min(3, 7, 5)", Val::Int(3));
}

#[test]
fn max() {
    assert("max(3, 7, 5)", Val::Int(7));
}

#[test]
fn clamp() {
    assert("clamp(9, -2, 23)", Val::Int(9));
    assert("clamp(-12, -5, 5)", Val::Int(-5));
    assert("clamp(31, 0, 7)", Val::Int(7));
}

#[test]
fn clamp_bounds() {
    assert_err(
        "clamp(0, 5, 4)",
        Error::InvalidClampBounds(
            ValRange::new(Val::Int(5), Range::pos(9)),
            ValRange::new(Val::Int(4), Range::pos(12)),
        ),
    );
}

#[test]
fn clamp_bounds_float() {
    assert_err(
        "clamp(0, 5.3, 4.5)",
        Error::InvalidClampBounds(
            ValRange::new(Val::Float(5.3), Range::of(9, 12)),
            ValRange::new(Val::Float(4.5), Range::of(14, 17)),
        ),
    );
}

#[test]
fn eq() {
    assert("false == false", Val::Bool(true));
    assert("false == true", Val::Bool(false));
    assert("2.0 == 2", Val::Bool(true));
    assert("4 == 4.2", Val::Bool(false));
    assert("7.8 == 7.8", Val::Bool(true));
    assert("5.1 == 5.12", Val::Bool(false));
}

#[test]
fn ne() {
    assert("false != false", Val::Bool(false));
    assert("false != true", Val::Bool(true));
    assert("2.0 != 2", Val::Bool(false));
    assert("4 != 4.2", Val::Bool(true));
    assert("7.8 != 7.8", Val::Bool(false));
    assert("5.1 != 5.12", Val::Bool(true));
}

#[test]
fn lt() {
    assert("2 < 5", Val::Bool(true));
    assert("3 < 3", Val::Bool(false));
    assert("8 < 7", Val::Bool(false));
}

#[test]
fn le() {
    assert("2 <= 5", Val::Bool(true));
    assert("3 <= 3", Val::Bool(true));
    assert("8 <= 7", Val::Bool(false));
}

#[test]
fn gt() {
    assert("2 > 5", Val::Bool(false));
    assert("3 > 3", Val::Bool(false));
    assert("8 > 7", Val::Bool(true));
}

#[test]
fn ge() {
    assert("2 >= 5", Val::Bool(false));
    assert("3 >= 3", Val::Bool(true));
    assert("8 >= 7", Val::Bool(true));
}

#[test]
fn or() {
    assert("false || false", Val::Bool(false));
    assert("false || true", Val::Bool(true));
    assert("true || false", Val::Bool(true));
    assert("true || true", Val::Bool(true));
}

#[test]
fn and() {
    assert("false && false", Val::Bool(false));
    assert("false && true", Val::Bool(false));
    assert("true && false", Val::Bool(false));
    assert("true && true", Val::Bool(true));
}

#[test]
fn bw_or() {
    assert("true | false", Val::Bool(true));
    assert("12 | 2", Val::Int(14));
}

#[test]
fn bw_and() {
    assert("true & false", Val::Bool(false));
    assert("6 & 12", Val::Int(4));
}

#[test]
fn not() {
    assert("!true", Val::Bool(false));
    assert("!false", Val::Bool(true));
}

#[test]
fn var() {
    assert("x = 7; x", Val::Int(7));
}

#[test]
fn unmatched_par() {
    assert_err(
        "4 ) + 5)",
        Error::UnexpectedParenthesis(Par::new(ParT::RoundClose, Range::pos(2))),
    );
}

#[test]
fn newline_sep() {
    assert("x = 7\n x", Val::Int(7));
}

#[test]
fn newline_ignored_after_op() {
    assert("x = 9 + \n 12 \n x", Val::Int(21));
}

#[test]
fn newline_ignored_before_op() {
    assert("x = 34\n - 45 \n x", Val::Int(-11));
}

#[test]
fn assertion() {
    assert_unit("assert(5 == 5)");
}

#[test]
fn assert_failed() {
    assert_err(
        "assert(4 == 5)",
        crate::Error::AssertFailed(Range::of(7, 13)),
    );
}

#[test]
fn assert_eq() {
    assert_unit("assert_eq(false, 4 == 3)");
}

#[test]
fn assert_eq_failed() {
    assert_err(
        "assert_eq(false, 5 == 5)",
        crate::Error::AssertEqFailed(
            ValRange::new(Val::Bool(false), Range::of(10, 15)),
            ValRange::new(Val::Bool(true), Range::of(17, 23)),
        ),
    );
}
