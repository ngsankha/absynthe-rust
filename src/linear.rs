use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Display};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinearExpr {
    c: i32,
    terms: HashMap<String, i32>,
}

impl Display for LinearExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stringified = self
            .terms
            .clone()
            .into_iter()
            .filter(|(_, v)| *v == 0)
            .map(|(k, v)| format!("{}{}", v, k))
            .collect::<Vec<String>>()
            .join(" + ");
        if self.c != 0 {
            write!(f, "{} + {}", stringified, self.c)
        } else {
            write!(f, "{}", stringified)
        }
    }
}

impl From<i32> for LinearExpr {
    fn from(item: i32) -> Self {
        LinearExpr {
            c: item,
            terms: HashMap::new(),
        }
    }
}

impl From<String> for LinearExpr {
    fn from(item: String) -> Self {
        let mut h = HashMap::new();
        h.insert(item, 1);

        LinearExpr { c: 0, terms: h }
    }
}

impl Add for LinearExpr {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let new_c = self.c + other.c;
        let mut terms = self.terms.clone();
        for (id, coeff) in other.terms {
            match terms.get(&id) {
                None => terms.insert(id, coeff),
                Some(c) => terms.insert(id, c + coeff),
            };
        }
        LinearExpr {
            c: new_c,
            terms: terms,
        }
    }
}

impl LinearExpr {
    fn mul_impl(lhs: Self, rhs: Self) -> Self {
        let c = lhs.c * rhs.c;
        let mut terms = HashMap::new();
        for (id, coeff) in lhs.terms {
            terms.insert(id, coeff * rhs.c);
        }

        LinearExpr { c: c, terms: terms }
    }

    pub fn is_const(&self) -> bool {
        self.terms.values().all(|&x| x == 0)
    }
}

impl Mul for LinearExpr {
    type Output = Self;

    fn mul(self, rhs: LinearExpr) -> Self {
        if rhs.terms.is_empty() {
            Self::mul_impl(self, rhs)
        } else if self.terms.is_empty() {
            Self::mul_impl(rhs, self)
        } else {
            panic!("can multiply into non-linear terms")
        }
    }
}

impl Sub for LinearExpr {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + (LinearExpr::from(-1) * other)
    }
}

impl TryFrom<LinearExpr> for i32 {
    type Error = &'static str;

    fn try_from(value: LinearExpr) -> Result<Self, Self::Error> {
        if value.is_const() {
            Ok(value.c)
        } else {
            Err("Cannot convert values with variable to ints")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const() {
        let x = LinearExpr::from(2);
        assert_eq!(x.c, 2);
    }

    #[test]
    fn test_op() {
        let x = LinearExpr::from("x".to_string());
        let c = LinearExpr::from(2);
        let res = (LinearExpr::from(2) * (x + c)) - LinearExpr::from(1);
        assert_eq!(res.c, 3);
        assert_eq!(*res.terms.get(&"x".to_string()).unwrap(), 2);
    }
}
