use crate::values::{Lattice, Value};
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;

pub type Term<T, U> = Rc<Expr<T, U>>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Func {
    Loc,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expr<T: Value, U: Lattice> {
    Const(T),
    Var(String),
    GetItem(Term<T, U>, Term<T, U>),
    List(Vec<Term<T, U>>),
    Call(Option<Term<T, U>>, Func),
    Hole(U, Option<Func>),
}

impl<T: Value + Display, U: Lattice> Display for Expr<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Const(c) => write!(f, "{}", c),
            Self::Var(var) => write!(f, "{}", var),
            Self::GetItem(recv, expr) => write!(f, "{}[{}]", recv, expr),
            Self::Call(recv, func) => match recv {
                None => write!(f, "{}", func),
                Some(recv) => write!(f, "{}.{}", recv, func),
            },
            Self::List(items) => write!(
                f,
                "[{}]",
                items
                    .into_iter()
                    .map(|v| format!("{}", v))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Hole(lat, _) => write!(f, "(□: {})", lat),
        }
    }
}

impl Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Loc => write!(f, "loc()"),
        }
    }
}

impl<T: Value, U: Lattice> Expr<T, U> {
    pub fn has_hole(&self) -> bool {
        match self {
            Self::GetItem(recv, expr) => recv.has_hole() || expr.has_hole(),
            Self::List(items) => items.into_iter().any(|v| v.has_hole()),
            Self::Call(None, f) => f.has_hole(),
            Self::Call(Some(recv), f) => recv.has_hole() || f.has_hole(),
            Self::Hole(_, _) => true,
            _ => false,
        }
    }

    pub fn size(&self) -> u32 {
        match self {
            Self::Call(None, f) => f.size(),
            Self::Call(Some(recv), f) => recv.size() + f.size(),
            Self::GetItem(recv, expr) => recv.size() + expr.size(),
            Self::List(items) => items.into_iter().map(|v| v.size()).sum(),
            Self::Hole(_, Some(f)) => f.size(),
            _ => 0,
        }
    }
}

impl Func {
    fn has_hole(&self) -> bool {
        match self {
            Self::Loc => false,
        }
    }

    fn size(&self) -> u32 {
        match self {
            Self::Loc => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concrete::PyVal;
    use crate::linear::LinearExpr;
    use crate::types::TypeLattice;

    #[test]
    fn converts_to_python_src() {
        let ast: Expr<PyVal, TypeLattice> = Expr::GetItem(
            Rc::new(Expr::Call(
                Some(Rc::new(Expr::Var("df".to_string()))),
                Func::Loc,
            )),
            Rc::new(Expr::List(vec![
                Rc::new(Expr::Const(PyVal::Int(LinearExpr::from(0)))),
                Rc::new(Expr::Const(PyVal::Int(LinearExpr::from(2)))),
                Rc::new(Expr::Const(PyVal::Int(LinearExpr::from(4)))),
            ])),
        );
        assert_eq!("df.loc()[[0, 2, 4]]", format!("{}", ast));
    }
}
