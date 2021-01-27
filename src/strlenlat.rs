use crate::linear::LinearExpr;
use crate::values::Lattice;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Display;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StrLenLat {
    Top,
    Len(LinearExpr),
    Bot,
}

impl Lattice for StrLenLat {
    fn top() -> Self {
        Self::Top
    }

    fn bot() -> Self {
        Self::Bot
    }
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

impl Add for StrLenLat {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (StrLenLat::Top, _) => StrLenLat::Top,
            (StrLenLat::Len(_), StrLenLat::Top) => StrLenLat::Top,
            (StrLenLat::Len(l1), StrLenLat::Len(l2)) => StrLenLat::Len(l1 + l2),
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
