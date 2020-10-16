use crate::concrete::*;
use std::collections::HashMap;

pub trait Lattice: PartialOrd + PartialEq {
    // fn meet(&self, other: &Self) -> Self;
    // fn join(&self, other: &Self) -> Self;
    fn top() -> Self;
    fn bot() -> Self;
}

#[derive(Debug, Clone, PartialEq)]
pub enum AbsValue<T: Lattice, U> {
    Abs(T),
    Conc(U),
}

impl<T: Lattice, U> AbsValue<T, U> {
    fn is_abstract(&self) -> bool {
        match self {
            AbsValue::Abs(_) => true,
            _ => false,
        }
    }

    fn is_concrete(&self) -> bool {
        !self.is_abstract()
    }

    pub fn top() -> Self {
        AbsValue::Abs(T::top())
    }

    pub fn bot() -> Self {
        AbsValue::Abs(T::bot())
    }
}

pub trait Value: Clone {
    fn error() -> Self;
}

pub trait Abstractable<T> {
    fn abstraction(&self) -> Option<T>;
}

pub trait Interpreter<T: Value, U: Lattice> {
    fn eval(expr: &Expr<T, U>, env: &HashMap<String, T>) -> Result<T, &'static str> {
        match expr {
            Expr::Const(v) => Ok(v.clone()),
            Expr::Var(x) => match env.get(x) {
                Some(val) => Ok(val.clone()),
                None => Ok(T::error()),
            },
            Expr::Call(call) => Ok(Self::eval_call(call, env)),
            _ => Err("holes cannot be processed"),
        }
    }

    fn eval_call(expr: &Func<T, U>, env: &HashMap<String, T>) -> T;
}
