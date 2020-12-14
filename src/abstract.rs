use crate::concrete::*;
use crate::interpreter::*;
use crate::linear::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Display};
use std::ops::Sub;

#[derive(Debug, Clone, PartialEq)]
pub enum StrLenLat {
    Top,
    Len(LinearExpr),
    Bot,
}

impl Display for StrLenLat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StrLenLat::Top => write!(f, "⊤"),
            StrLenLat::Len(e) => write!(f, "{}", e),
            StrLenLat::Bot => write!(f, "⊥"),
        }
    }
}

impl From<LinearExpr> for StrLenLat {
    fn from(item: LinearExpr) -> Self {
        StrLenLat::Len(item)
    }
}

impl From<i32> for StrLenLat {
    fn from(item: i32) -> Self {
        StrLenLat::from(LinearExpr::from(item))
    }
}

impl From<String> for StrLenLat {
    fn from(item: String) -> Self {
        StrLenLat::from(LinearExpr::from(item))
    }
}

impl Sub for StrLenLat {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (StrLenLat::Top, _) => StrLenLat::Top,
            (StrLenLat::Len(_), StrLenLat::Top) => StrLenLat::Top,
            (StrLenLat::Len(l1), StrLenLat::Len(l2)) => StrLenLat::Len(l1 - l2),
            (StrLenLat::Len(_), StrLenLat::Bot) => StrLenLat::Bot,
            (StrLenLat::Bot, _) => StrLenLat::Bot,
        }
    }
}

impl PartialOrd for StrLenLat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            StrLenLat::Top => match other {
                StrLenLat::Top => Some(Ordering::Equal),
                _ => Some(Ordering::Greater),
            },
            StrLenLat::Len(l1) => match other {
                StrLenLat::Top => Some(Ordering::Less),
                StrLenLat::Len(l2) => {
                    if l1 == l2 {
                        Some(Ordering::Equal)
                    } else {
                        None
                    }
                }
                StrLenLat::Bot => Some(Ordering::Greater),
            },
            StrLenLat::Bot => match other {
                StrLenLat::Bot => Some(Ordering::Equal),
                _ => Some(Ordering::Less),
            },
        }
    }
}

impl Lattice for StrLenLat {
    fn top() -> Self {
        Self::Top
    }

    fn bot() -> Self {
        Self::Bot
    }
}

impl Abstractable<StrLenLat> for StrVal {
    fn abstraction(&self) -> Option<StrLenLat> {
        match self {
            StrVal::Str(s) => Some(StrLenLat::Len(LinearExpr::from(s.chars().count() as i32))),
            _ => None,
        }
    }
}

pub type AbsStrVal = AbsValue<StrLenLat, StrVal>;

impl PartialOrd for AbsStrVal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (AbsStrVal::Abs(a1), AbsStrVal::Abs(a2)) => a1.partial_cmp(a2),
            _ => None,
        }
    }
}

impl Lattice for AbsStrVal {
    fn top() -> Self {
        Self::Abs(StrLenLat::top())
    }

    fn bot() -> Self {
        Self::Abs(StrLenLat::bot())
    }
}

impl From<StrVal> for AbsStrVal {
    fn from(item: StrVal) -> Self {
        AbsValue::Conc(item)
    }
}

impl From<StrLenLat> for AbsStrVal {
    fn from(item: StrLenLat) -> Self {
        AbsValue::Abs(item)
    }
}

impl Value for AbsStrVal {
    fn error() -> Self {
        AbsValue::Conc(StrVal::Error)
    }
}

impl AbsStrVal {
    fn abstractable(&self) -> bool {
        match self {
            AbsStrVal::Conc(StrVal::Str(_)) => true,
            _ => false,
        }
    }
}

pub struct StrLenInterp;

impl Interpreter<AbsStrVal, StrLenLat> for StrLenInterp {
    fn eval(
        expr: &Expr<AbsStrVal, StrLenLat>,
        env: &HashMap<String, AbsStrVal>,
    ) -> Result<AbsStrVal, &'static str> {
        match expr {
            Expr::Const(v) => Ok(v.clone()),
            Expr::Var(x) => match env.get(x) {
                Some(val) => Ok(val.clone()),
                None => Ok(AbsStrVal::error()),
            },
            Expr::Call(call) => Ok(Self::eval_call(call, env)),
            Expr::Hole(v, _) => Ok(AbsStrVal::Abs(v.clone())),
            _ => unreachable!(),
        }
    }

    fn eval_call(expr: &Func<AbsStrVal, StrLenLat>, env: &HashMap<String, AbsStrVal>) -> AbsStrVal {
        match expr {
            Func::Append(s1, s2) => {
                Self::abs_str_append(Self::eval(s1, env).unwrap(), Self::eval(s2, env).unwrap())
            }
            Func::Replace(s1, s2, s3) => Self::abs_str_replace(
                Self::eval(s1, env).unwrap(),
                Self::eval(s2, env).unwrap(),
                Self::eval(s3, env).unwrap(),
            ),
            Func::Substr(s1, s2, s3) => Self::abs_str_substr(
                Self::eval(s1, env).unwrap(),
                Self::eval(s2, env).unwrap(),
                Self::eval(s3, env).unwrap(),
            ),
            Func::Add(i, j) => {
                Self::abs_int_add(Self::eval(i, env).unwrap(), Self::eval(j, env).unwrap())
            }
            Func::Sub(i, j) => {
                Self::abs_int_sub(Self::eval(i, env).unwrap(), Self::eval(j, env).unwrap())
            }
            Func::Len(s) => Self::abs_str_len(Self::eval(s, env).unwrap()),
        }
    }
}

impl StrLenInterp {
    fn abs_str_append(v1: AbsStrVal, v2: AbsStrVal) -> AbsStrVal {
        match (v1.clone(), v2.clone()) {
            (AbsStrVal::Conc(c1), AbsStrVal::Conc(c2)) => match (c1.clone(), c2.clone()) {
                (StrVal::Str(s1), StrVal::Str(s2)) => AbsStrVal::Conc(StrVal::Str(s1 + &s2)),
                (_, _) => AbsStrVal::error(),
            },
            (AbsStrVal::Conc(c), AbsStrVal::Abs(_)) => {
                let abs_v1 = c.abstraction();
                if abs_v1.is_none() {
                    AbsStrVal::error()
                } else {
                    Self::abs_str_append(AbsStrVal::Abs(abs_v1.unwrap()), v2)
                }
            }
            (AbsStrVal::Abs(_), AbsStrVal::Conc(c)) => {
                let abs_v2 = c.abstraction();
                if abs_v2.is_none() {
                    AbsStrVal::error()
                } else {
                    Self::abs_str_append(v1, AbsStrVal::Abs(abs_v2.unwrap()))
                }
            }
            (AbsStrVal::Abs(s1), AbsStrVal::Abs(s2)) => match (s1, s2) {
                (StrLenLat::Top, _) => AbsStrVal::top(),
                (_, StrLenLat::Top) => AbsStrVal::top(),
                (StrLenLat::Len(l1), StrLenLat::Len(l2)) => AbsStrVal::Abs(StrLenLat::Len(l1 + l2)),
                (StrLenLat::Len(l1), StrLenLat::Bot) => AbsStrVal::Abs(StrLenLat::Len(l1)),
                (StrLenLat::Bot, StrLenLat::Len(l2)) => AbsStrVal::Abs(StrLenLat::Len(l2)),
                (_, _) => AbsStrVal::bot(),
            },
        }
    }

    fn abs_str_replace(v1: AbsStrVal, v2: AbsStrVal, v3: AbsStrVal) -> AbsStrVal {
        match (v1.clone(), v2.clone(), v3.clone()) {
            (AbsStrVal::Conc(c1), AbsStrVal::Conc(c2), AbsStrVal::Conc(c3)) => match (c1, c2, c3) {
                (StrVal::Str(s1), StrVal::Str(s2), StrVal::Str(s3)) => {
                    AbsStrVal::Conc(StrVal::Str(s1.replace(&s2, &s3)))
                }
                _ => AbsStrVal::error(),
            },
            (AbsStrVal::Abs(_), AbsStrVal::Abs(_), AbsStrVal::Abs(_)) => AbsStrVal::top(),
            _ => {
                if v1.abstractable() && v2.abstractable() && v3.abstractable() {
                    AbsStrVal::top()
                } else {
                    AbsStrVal::error()
                }
            }
        }
    }

    fn abs_str_substr(v1: AbsStrVal, v2: AbsStrVal, v3: AbsStrVal) -> AbsStrVal {
        match (v1, v2, v3) {
            (AbsStrVal::Conc(c1), AbsStrVal::Conc(c2), AbsStrVal::Conc(c3)) => match (c1, c2, c3) {
                (StrVal::Str(s1), StrVal::Int(start), StrVal::Int(end)) => {
                    if start.is_const() && end.is_const() {
                        AbsStrVal::Conc(StrVal::Str(
                            s1.chars()
                                .skip(i32::try_from(start.clone()).unwrap() as usize)
                                .take(i32::try_from(end - start).unwrap() as usize)
                                .collect(),
                        ))
                    } else {
                        AbsStrVal::error()
                    }
                }
                _ => AbsStrVal::error(),
            },
            (_, AbsStrVal::Conc(StrVal::Int(start)), AbsStrVal::Conc(StrVal::Int(end))) => {
                AbsStrVal::Abs(StrLenLat::Len(LinearExpr::from(end - start)))
            }
            _ => AbsStrVal::error(),
        }
    }

    fn abs_int_add(v1: AbsStrVal, v2: AbsStrVal) -> AbsStrVal {
        match (v1, v2) {
            (AbsStrVal::Conc(c1), AbsStrVal::Conc(c2)) => match (c1, c2) {
                (StrVal::Int(i), StrVal::Int(j)) => AbsStrVal::Conc(StrVal::Int(i + j)),
                _ => AbsStrVal::error(),
            },
            _ => AbsStrVal::error(),
        }
    }

    fn abs_int_sub(v1: AbsStrVal, v2: AbsStrVal) -> AbsStrVal {
        match (v1, v2) {
            (AbsStrVal::Conc(c1), AbsStrVal::Conc(c2)) => match (c1, c2) {
                (StrVal::Int(i), StrVal::Int(j)) => AbsStrVal::Conc(StrVal::Int(i - j)),
                _ => AbsStrVal::error(),
            },
            _ => AbsStrVal::error(),
        }
    }

    fn abs_str_len(v: AbsStrVal) -> AbsStrVal {
        match v {
            AbsStrVal::Conc(c) => match c {
                StrVal::Str(s) => {
                    AbsStrVal::Conc(StrVal::Int(LinearExpr::from(s.chars().count() as i32)))
                }
                _ => AbsStrVal::error(),
            },
            AbsStrVal::Abs(StrLenLat::Len(l)) => AbsStrVal::Conc(StrVal::Int(l)),
            _ => AbsStrVal::error(),
        }
    }
}
