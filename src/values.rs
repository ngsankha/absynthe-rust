use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::Display;

pub trait Lattice: PartialOrd + PartialEq + Clone + Display {
    // fn meet(&self, other: &Self) -> Self;
    // fn join(&self, other: &Self) -> Self;
    fn top() -> Self;
    fn bot() -> Self;
}

pub trait Value: Clone + Display + PartialEq {
    fn is_abstract(&self) -> bool;

    fn is_concrete(&self) -> bool {
        !self.is_abstract()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MixedValue<T: Value, U: Lattice> {
    Conc(T),
    Abs(U),
}

impl<T: Value, U: Lattice> Value for MixedValue<T, U> {
    fn is_abstract(&self) -> bool {
        match self {
            Self::Abs(_) => true,
            _ => false,
        }
    }
}

impl<T: Value, U: Lattice> Display for MixedValue<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Abs(v) => write!(f, "α({})", v),
            Self::Conc(v) => write!(f, "{}", v),
        }
    }
}

impl<T: Value, U: Lattice + TryFrom<T>> PartialOrd for MixedValue<T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Conc(_), Self::Conc(_)) => None,
            (Self::Conc(c), Self::Abs(a)) => U::try_from(c.clone())
                .ok()
                .and_then(|c2a| c2a.partial_cmp(a)),
            (Self::Abs(a), Self::Conc(c)) => U::try_from(c.clone())
                .ok()
                .and_then(|c2a| a.partial_cmp(&c2a)),
            (Self::Abs(a1), Self::Abs(a2)) => a1.partial_cmp(a2),
        }
    }
}

impl<T: Value, U: Lattice> MixedValue<T, U> {
    pub fn from_concrete(item: T) -> MixedValue<T, U> {
        Self::Conc(item)
    }

    pub fn from_abstract(item: U) -> MixedValue<T, U> {
        Self::Abs(item)
    }
}
