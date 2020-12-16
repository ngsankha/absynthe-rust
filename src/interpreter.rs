use crate::environment::Environment;
use crate::values::{Lattice, Value};
use std::fmt;
use std::fmt::{Debug, Display};

#[derive(Debug, Clone, PartialEq)]
pub enum AbsValue<T: Lattice, U> {
    Abs(T),
    Conc(U),
}

impl<T: Lattice + Display, U: Display> Display for AbsValue<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AbsValue::Abs(v) => write!(f, "{}", v),
            AbsValue::Conc(v) => write!(f, "{}", v),
        }
    }
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

pub trait Abstractable<T> {
    fn abstraction(&self) -> Option<T>;
}

pub trait Evaluable<T: Value + Debug, U: Lattice> {
    fn eval(&self, env: &Environment<T>) -> EvalResult<T>;
}

pub type EvalResult<T: Value> = Result<T, &'static str>;
