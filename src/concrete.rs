use crate::interpreter::*;
use crate::r#abstract::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StrVal {
    Str(String),
    Int(i32),
    Error,
}

#[derive(Debug, Clone)]
pub enum Func<T: Value, U: Lattice> {
    Append(Expr<T, U>, Expr<T, U>),
    Replace(Expr<T, U>, Expr<T, U>, Expr<T, U>),
    Substr(Expr<T, U>, Expr<T, U>, Expr<T, U>),
    Add(Expr<T, U>, Expr<T, U>),
    Sub(Expr<T, U>, Expr<T, U>),
    Len(Expr<T, U>),
}

#[derive(Debug, Clone)]
pub enum Expr<T: Value, U: Lattice> {
    Const(T),
    Var(String),
    Call(Box<Func<T, U>>),
    Hole(U, Option<Box<Func<T, U>>>),
    ConcHole(u32),
    DepHole,
}

impl<T: Value, U: Lattice> Expr<T, U> {
    pub fn has_hole(&self) -> bool {
        match self {
            Self::Hole(_, _) => true,
            Self::ConcHole(_) => true,
            Self::DepHole => true,
            Self::Const(_) => false,
            Self::Var(_) => false,
            Self::Call(f) => match &*(*f) {
                Func::Append(arg1, arg2) => arg1.has_hole() || arg2.has_hole(),
                Func::Replace(arg1, arg2, arg3) => {
                    arg1.has_hole() || arg2.has_hole() || arg3.has_hole()
                }
                Func::Substr(arg1, arg2, arg3) => {
                    arg1.has_hole() || arg2.has_hole() || arg3.has_hole()
                }
                Func::Add(arg1, arg2) => arg1.has_hole() || arg2.has_hole(),
                Func::Sub(arg1, arg2) => arg1.has_hole() || arg2.has_hole(),
                Func::Len(arg) => arg.has_hole(),
            },
        }
    }

    pub fn size(&self) -> u32 {
        match self {
            Self::Call(f) => f.size(),
            Self::Hole(_, Some(f)) => f.size(),
            Self::ConcHole(s) => *s,
            _ => 0,
        }
    }
}

impl<T: Value, U: Lattice> Func<T, U> {
    fn size(&self) -> u32 {
        (match self {
            Func::Append(arg1, arg2) => arg1.size() + arg2.size(),
            Func::Replace(arg1, arg2, arg3) => arg1.size() + arg2.size() + arg3.size(),
            Func::Substr(arg1, arg2, arg3) => arg1.size() + arg2.size() + arg3.size(),
            Func::Add(arg1, arg2) => arg1.size() + arg2.size(),
            Func::Sub(arg1, arg2) => arg1.size() + arg2.size(),
            Func::Len(arg) => arg.size(),
        }) + 1
    }
}

impl Value for StrVal {
    fn error() -> Self {
        StrVal::Error
    }
}

impl From<String> for StrVal {
    fn from(item: String) -> Self {
        StrVal::Str(item)
    }
}

impl From<i32> for StrVal {
    fn from(item: i32) -> Self {
        StrVal::Int(item)
    }
}

pub struct StrOpInterpreter;

impl Interpreter<StrVal, StrLenLat> for StrOpInterpreter {
    fn eval_call(call: &Func<StrVal, StrLenLat>, env: &HashMap<String, StrVal>) -> StrVal {
        match call {
            Func::Append(s1, s2) => {
                Self::str_append(Self::eval(s1, env).unwrap(), Self::eval(s2, env).unwrap())
            }
            Func::Replace(s1, s2, s3) => Self::str_replace(
                Self::eval(s1, env).unwrap(),
                Self::eval(s2, env).unwrap(),
                Self::eval(s3, env).unwrap(),
            ),
            Func::Substr(s1, s2, s3) => Self::str_substr(
                Self::eval(s1, env).unwrap(),
                Self::eval(s2, env).unwrap(),
                Self::eval(s3, env).unwrap(),
            ),
            Func::Add(i, j) => {
                Self::int_add(Self::eval(i, env).unwrap(), Self::eval(j, env).unwrap())
            }
            Func::Sub(i, j) => {
                Self::int_sub(Self::eval(i, env).unwrap(), Self::eval(j, env).unwrap())
            }
            Func::Len(s) => Self::str_len(Self::eval(s, env).unwrap()),
        }
    }
}

impl StrOpInterpreter {
    fn str_append(v1: StrVal, v2: StrVal) -> StrVal {
        match (v1, v2) {
            (StrVal::Str(s1), StrVal::Str(s2)) => StrVal::Str(s1 + &s2),
            _ => StrVal::error(),
        }
    }

    fn str_replace(v1: StrVal, v2: StrVal, v3: StrVal) -> StrVal {
        match (v1, v2, v3) {
            (StrVal::Str(s1), StrVal::Str(s2), StrVal::Str(s3)) => {
                StrVal::Str(s1.replace(&s2, &s3))
            }
            _ => StrVal::error(),
        }
    }

    fn str_substr(v1: StrVal, v2: StrVal, v3: StrVal) -> StrVal {
        match (v1, v2, v3) {
            (StrVal::Str(s1), StrVal::Int(start), StrVal::Int(end)) => StrVal::Str(
                s1.chars()
                    .skip(start as usize)
                    .take((end - start) as usize)
                    .collect(),
            ),
            _ => StrVal::error(),
        }
    }

    fn int_add(v1: StrVal, v2: StrVal) -> StrVal {
        match (v1, v2) {
            (StrVal::Int(i), StrVal::Int(j)) => StrVal::Int(i + j),
            _ => StrVal::error(),
        }
    }

    fn int_sub(v1: StrVal, v2: StrVal) -> StrVal {
        match (v1, v2) {
            (StrVal::Int(i), StrVal::Int(j)) => StrVal::Int(i - j),
            _ => StrVal::error(),
        }
    }

    fn str_len(v: StrVal) -> StrVal {
        match v {
            StrVal::Str(s) => StrVal::Int(s.chars().count() as i32),
            _ => StrVal::error(),
        }
    }
}
