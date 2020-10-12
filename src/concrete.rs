use std::collections::HashMap;
use crate::interpreter::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StrVal {
    Str(String),
    Int(i32),
    Error
}

#[derive(Debug)]
pub enum Func<T> {
    Append(Expr<T>, Expr<T>),
    Replace(Expr<T>, Expr<T>, Expr<T>),
    Substr(Expr<T>, Expr<T>, Expr<T>),
    Add(Expr<T>, Expr<T>),
    Sub(Expr<T>, Expr<T>),
    Len(Expr<T>)
}

#[derive(Debug)]
pub enum Expr<T> {
    Const(T),
    Var(String),
    Call(Box<Func<T>>)
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

pub struct StrOpInterpreter;

impl Interpreter<StrVal> for StrOpInterpreter {
    fn eval_call(call: &Func<StrVal>, env: &HashMap<String, StrVal>) -> StrVal {
        match call {
            Func::Append(s1, s2) => Self::str_append(Self::eval(s1, env), Self::eval(s2, env)),
            Func::Replace(s1, s2, s3) => Self::str_replace(Self::eval(s1, env), Self::eval(s2, env), Self::eval(s3, env)),
            Func::Substr(s1, s2, s3) => Self::str_substr(Self::eval(s1, env), Self::eval(s2, env), Self::eval(s3, env)),
            Func::Add(i, j) => Self::int_add(Self::eval(i, env), Self::eval(j, env)),
            Func::Sub(i, j) => Self::int_sub(Self::eval(i, env), Self::eval(j, env)),
            Func::Len(s) => Self::str_len(Self::eval(s, env))
        }
    }
}

impl StrOpInterpreter {
    fn str_append(v1: StrVal, v2: StrVal) -> StrVal {
        match (v1, v2) {
            (StrVal::Str(s1), StrVal::Str(s2)) => StrVal::Str(s1 + &s2),
            _ => StrVal::error()
        }
    }

    fn str_replace(v1: StrVal, v2: StrVal, v3: StrVal) -> StrVal {
        match (v1, v2, v3) {
            (StrVal::Str(s1), StrVal::Str(s2), StrVal::Str(s3)) => StrVal::Str(s1.replace(&s2, &s3)),
            _ => StrVal::error()
        }
    }

    fn str_substr(v1: StrVal, v2: StrVal, v3: StrVal) -> StrVal {
        match (v1, v2, v3) {
            (StrVal::Str(s1), StrVal::Int(start), StrVal::Int(end)) => StrVal::Str(s1.chars().skip(start as usize).take((end - start) as usize).collect()),
            _ => StrVal::error()
        }
    }

    fn int_add(v1: StrVal, v2: StrVal) -> StrVal {
        match (v1, v2) {
            (StrVal::Int(i), StrVal::Int(j)) => StrVal::Int(i + j),
            _ => StrVal::error()
        }
    }

    fn int_sub(v1: StrVal, v2: StrVal) -> StrVal {
        match (v1, v2) {
            (StrVal::Int(i), StrVal::Int(j)) => StrVal::Int(i - j),
            _ => StrVal::error()
        }
    }

    fn str_len(v: StrVal) -> StrVal {
        match v {
            StrVal::Str(s) => StrVal::Int(s.chars().count() as i32),
            _ => StrVal::error()
        }
    }
}
