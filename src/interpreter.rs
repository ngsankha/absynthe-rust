use std::collections::HashMap;
use crate::concrete::*;

pub trait Lattice: PartialOrd + PartialEq {
    // fn meet(&self, other: &Self) -> Self;
    fn join(&self, other: &Self) -> Self;
    fn top() -> Self;
    fn bot() -> Self;
}

#[derive(Debug, Clone, PartialEq)]
pub enum AbsValue<T: Lattice, U> {
    Abs(T),
    Conc(U)
}

impl<T: Lattice, U> AbsValue<T, U> {
    fn is_abstract(&self) -> bool {
        match self {
            AbsValue::Abs(_) => true,
            _ => false
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

pub trait Interpreter<T: Value> {
    fn eval(expr: &Expr<T>, env: &HashMap<String, T>) -> T {
        match expr {
            Expr::Const(v) => v.clone(),
            Expr::Var(x) => match env.get(x) {
                Some(val) => val.clone(),
                None => T::error()
            }
            Expr::Call(call) => Self::eval_call(call, env)
        }
    }

    fn eval_call(expr: &Func<T>, env: &HashMap<String, T>) -> T;
}
