use std::iter::Peekable;
use std::str::Chars;

use crate::{CRange, Context};

pub use token::*;

mod str;
#[cfg(test)]
mod test;
mod token;

struct Lexer<'a> {
    tokens: Vec<Token>,
    literal: String,
    chars: Peekable<Chars<'a>>,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            tokens: Vec::new(),
            literal: String::new(),
            chars: input.chars().peekable(),
            cursor: 0,
        }
    }

    fn next(&mut self) -> Option<char> {
        self.cursor += 1;
        self.chars.next()
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn next_if(&mut self, expected: char) -> Option<char> {
        if let Some(c) = self.peek() {
            if c == expected {
                return self.next();
            }
        }

        None
    }

    const fn pos(&self) -> usize {
        self.cursor.saturating_sub(1)
    }
}

impl Context {
    pub fn lex(&mut self, string: &str) -> crate::Result<Vec<Token>> {
        let mut lexer = Lexer::new(string);

        while let Some(c) = lexer.next() {
            let range = CRange::pos(lexer.pos());
            match c {
                '"' => self.string_literal(&mut lexer)?,
                ' ' | '\r' => self.end_literal(&mut lexer)?,
                '\n' => self.new_atom(&mut lexer, Token::pct(PctT::Newln, range))?,
                '+' => self.two_char_op(&mut lexer, OpT::Add, OpT::AddAssign, '=')?,
                '-' | '−' => self.two_char_op(&mut lexer, OpT::Sub, OpT::SubAssign, '=')?,
                '*' | '×' => self.two_char_op(&mut lexer, OpT::Mul, OpT::MulAssign, '=')?,
                '/' | '÷' => self.two_char_op(&mut lexer, OpT::Div, OpT::DivAssign, '=')?,
                '%' => self.new_atom(&mut lexer, Token::op(OpT::Rem, range))?,
                '^' => self.new_atom(&mut lexer, Token::op(OpT::Pow, range))?,
                '=' => self.two_char_op(&mut lexer, OpT::Assign, OpT::Eq, '=')?,
                '.' => match lexer.peek() {
                    Some('.') => {
                        lexer.next();
                        let op = match lexer.next_if('=') {
                            Some(_) => OpT::RangeIn,
                            None => OpT::RangeEx,
                        };
                        let r = CRange::of(range.start, lexer.pos() + 1);
                        self.new_atom(&mut lexer, Token::op(op, r))?;
                    }
                    Some(c) if c.is_digit(10) => lexer.literal.push('.'),
                    _ => self.new_atom(&mut lexer, Token::op(OpT::Dot, range))?,
                },
                '<' => self.two_char_op(&mut lexer, OpT::Lt, OpT::Le, '=')?,
                '>' => self.two_char_op(&mut lexer, OpT::Gt, OpT::Ge, '=')?,
                '|' => self.two_char_op(&mut lexer, OpT::BwOr, OpT::Or, '|')?,
                '&' => self.two_char_op(&mut lexer, OpT::BwAnd, OpT::And, '&')?,
                '!' => self.two_char_op(&mut lexer, OpT::Bang, OpT::Ne, '=')?,
                '(' => self.new_atom(&mut lexer, Token::par(ParT::RoundOpen, range))?,
                '[' => self.new_atom(&mut lexer, Token::par(ParT::SquareOpen, range))?,
                '{' => self.new_atom(&mut lexer, Token::par(ParT::CurlyOpen, range))?,
                ')' => self.new_atom(&mut lexer, Token::par(ParT::RoundClose, range))?,
                ']' => self.new_atom(&mut lexer, Token::par(ParT::SquareClose, range))?,
                '}' => self.new_atom(&mut lexer, Token::par(ParT::CurlyClose, range))?,
                ',' => self.new_atom(&mut lexer, Token::pct(PctT::Comma, range))?,
                ';' => self.new_atom(&mut lexer, Token::pct(PctT::Semi, range))?,
                c => lexer.literal.push(c),
            }
        }

        self.end_literal(&mut lexer)?;

        Ok(lexer.tokens)
    }

    fn new_atom(&mut self, lexer: &mut Lexer<'_>, token: Token) -> crate::Result<()> {
        self.end_literal(lexer)?;
        lexer.tokens.push(token);
        Ok(())
    }

    fn two_char_op(
        &mut self,
        lexer: &mut Lexer<'_>,
        one: OpT,
        two: OpT,
        expected: char,
    ) -> crate::Result<()> {
        match lexer.next_if(expected) {
            Some(_) => {
                let r = CRange::of(lexer.pos() - 1, lexer.pos() + 1);
                self.new_atom(lexer, Token::op(two, r))
            }
            None => {
                let r = CRange::pos(lexer.pos());
                self.new_atom(lexer, Token::op(one, r))
            }
        }
    }

    fn end_literal(&mut self, lexer: &mut Lexer<'_>) -> crate::Result<()> {
        if lexer.literal.is_empty() {
            return Ok(());
        }

        let start = lexer.pos() - lexer.literal.chars().count();
        let range = CRange::of(start, lexer.pos());

        let literal = lexer.literal.as_str();
        let token = match literal {
            "true" => Token::expr(ExprT::bool(true), range),
            "false" => Token::expr(ExprT::bool(false), range),
            "div" => Token::op(OpT::IntDiv, range),
            "mod" => Token::op(OpT::Rem, range),
            "if" => Token::kw(KwT::If, range),
            "else" => Token::kw(KwT::Else, range),
            "while" => Token::kw(KwT::While, range),
            "for" => Token::kw(KwT::For, range),
            "in" => Token::kw(KwT::In, range),
            "fun" => Token::kw(KwT::Fun, range),
            "val" => Token::kw(KwT::Val, range),
            "var" => Token::kw(KwT::Var, range),
            _ => {
                if literal.chars().next().unwrap().is_digit(10) {
                    let num = if let Ok(i) = literal.parse::<i128>() {
                        ExprT::int(i)
                    } else if let Ok(f) = literal.parse::<f64>() {
                        ExprT::float(f)
                    } else {
                        return Err(crate::Error::InvalidNumberFormat(range));
                    };
                    Token::expr(num, range)
                } else {
                    for (i, c) in literal.char_indices() {
                        match c {
                            '0'..='9' => (),
                            'a'..='z' => (),
                            'A'..='Z' => (),
                            '_' => (),
                            _ => {
                                return Err(crate::Error::InvalidChar(CRange::pos(range.start + i)))
                            }
                        }
                    }

                    let id = self.idents.push(literal);
                    Token::expr(ExprT::Ident(id), range)
                }
            }
        };

        lexer.literal.clear();
        lexer.tokens.push(token);

        Ok(())
    }

    fn string_literal(&mut self, lexer: &mut Lexer<'_>) -> crate::Result<()> {
        self.end_literal(lexer)?;

        let start = lexer.pos();
        while let Some(c) = lexer.next() {
            match c {
                '"' => {
                    self.end_string_literal(lexer, start)?;
                    return Ok(());
                }
                '\\' if lexer.peek() == Some('\n') => {
                    lexer.next();

                    while let Some(c) = lexer.peek() {
                        if c.is_ascii_whitespace() {
                            lexer.next();
                        } else {
                            break;
                        }
                    }
                }
                '\\' => match self.escape_char(lexer) {
                    Ok(c) => lexer.literal.push(c),
                    Err(e) => {
                        if e.fail {
                            if e.end_str {
                                self.end_string_literal(lexer, start)?;
                            }
                            return Err(e.error);
                        }

                        self.errors.push(e.error);
                        if e.end_str {
                            self.end_string_literal(lexer, start)?;
                            return Ok(());
                        }
                    }
                },
                _ => lexer.literal.push(c),
            }
        }

        let r = CRange::pos(start);
        Err(crate::Error::MissingClosingQuote(r))
    }

    fn end_string_literal(&mut self, lexer: &mut Lexer<'_>, start: usize) -> crate::Result<()> {
        let str = Val::Str(lexer.literal.clone());
        let range = CRange::of(start, lexer.pos() + 1);
        lexer.tokens.push(Token::expr(ExprT::Val(str), range));
        lexer.literal.clear();
        Ok(())
    }
}
