use crate::values::{Lattice, Value};
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;

pub type Term<T, U> = Rc<Expr<T, U>>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Func<T: Value, U: Lattice> {
    Append(Term<T, U>, Term<T, U>),
    Replace(Term<T, U>, Term<T, U>, Term<T, U>),
    Substr(Term<T, U>, Term<T, U>, Term<T, U>),
    Add(Term<T, U>, Term<T, U>),
    Sub(Term<T, U>, Term<T, U>),
    Len(Term<T, U>),
    At(Term<T, U>, Term<T, U>),
    ToStr(Term<T, U>),
    ToInt(Term<T, U>),
    IndexOf(Term<T, U>, Term<T, U>, Term<T, U>),
    PrefixOf(Term<T, U>, Term<T, U>),
    SuffixOf(Term<T, U>, Term<T, U>),
    Contains(Term<T, U>, Term<T, U>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expr<T: Value, U: Lattice> {
    Const(T),
    Var(String),
    Call(Func<T, U>),
    If(Term<T, U>, Term<T, U>, Term<T, U>),
    Hole(U, Option<Func<T, U>>),
    ConcHole(u32),
    DepHole,
}

impl<T: Value + Display, U: Lattice> Display for Expr<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Const(c) => write!(f, "{}", c),
            Self::Var(var) => write!(f, "{}", var),
            Self::Call(func) => write!(f, "{}", func),
            Self::If(cond, then, otherwise) => write!(f, "(if {} {} {})", cond, then, otherwise),
            Self::Hole(lat, _) => write!(f, "(□: {})", lat),
            Self::DepHole => write!(f, "□"),
            Self::ConcHole(size) => write!(f, "□({})", size),
        }
    }
}

impl<T: Value + Display, U: Lattice> Display for Func<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Append(arg1, arg2) => write!(f, "(append {} {})", arg1, arg2),
            Self::Replace(arg1, arg2, arg3) => write!(f, "(replace {} {} {})", arg1, arg2, arg3),
            Self::Substr(arg1, arg2, arg3) => write!(f, "(substr {} {} {})", arg1, arg2, arg3),
            Self::Add(arg1, arg2) => write!(f, "(+ {} {})", arg1, arg2),
            Self::Sub(arg1, arg2) => write!(f, "(- {} {})", arg1, arg2),
            Self::Len(arg1) => write!(f, "(len {})", arg1),
            Self::At(arg1, arg2) => write!(f, "(at {} {})", arg1, arg2),
            Self::ToStr(arg) => write!(f, "(to-str {})", arg),
            Self::ToInt(arg) => write!(f, "(to-int {})", arg),
            Self::IndexOf(arg1, arg2, arg3) => write!(f, "(indexof {} {} {})", arg1, arg2, arg3),
            Self::PrefixOf(arg1, arg2) => write!(f, "(prefixof {} {})", arg1, arg2),
            Self::SuffixOf(arg1, arg2) => write!(f, "(suffixof {} {})", arg1, arg2),
            Self::Contains(arg1, arg2) => write!(f, "(contains {} {})", arg1, arg2),
        }
    }
}

impl<T: Value, U: Lattice> Expr<T, U> {
    pub fn has_hole(&self) -> bool {
        match self {
            Self::Hole(_, _) => true,
            Self::ConcHole(_) => unreachable!(),
            Self::DepHole => unreachable!(),
            Self::Const(_) => false,
            Self::Var(_) => false,
            Self::Call(f) => f.has_hole(),
            Self::If(cond, then, otherwise) => {
                cond.has_hole() || then.has_hole() || otherwise.has_hole()
            }
        }
    }

    pub fn size(&self) -> u32 {
        match self {
            Self::Call(f) => f.size(),
            Self::If(cond, then, otherwise) => cond.size() + then.size() + otherwise.size(),
            Self::Hole(_, Some(f)) => f.size(),
            Self::ConcHole(s) => *s,
            _ => 0,
        }
    }
}

impl<T: Value, U: Lattice> Func<T, U> {
    fn has_hole(&self) -> bool {
        match self {
            Self::Append(arg1, arg2) => arg1.has_hole() || arg2.has_hole(),
            Self::Replace(arg1, arg2, arg3) => {
                arg1.has_hole() || arg2.has_hole() || arg3.has_hole()
            }
            Self::Substr(arg1, arg2, arg3) => arg1.has_hole() || arg2.has_hole() || arg3.has_hole(),
            Self::Add(arg1, arg2) => arg1.has_hole() || arg2.has_hole(),
            Self::Sub(arg1, arg2) => arg1.has_hole() || arg2.has_hole(),
            Self::Len(arg) => arg.has_hole(),
            Self::At(arg1, arg2) => arg1.has_hole() || arg2.has_hole(),
            Self::ToStr(arg) => arg.has_hole(),
            Self::ToInt(arg) => arg.has_hole(),
            Self::IndexOf(arg1, arg2, arg3) => {
                arg1.has_hole() || arg2.has_hole() || arg3.has_hole()
            }
            Self::PrefixOf(arg1, arg2) => arg1.has_hole() || arg2.has_hole(),
            Self::SuffixOf(arg1, arg2) => arg1.has_hole() || arg2.has_hole(),
            Self::Contains(arg1, arg2) => arg1.has_hole() || arg2.has_hole(),
        }
    }

    fn size(&self) -> u32 {
        match self {
            Self::Append(arg1, arg2) => arg1.size() + arg2.size() + 2,
            Self::Replace(arg1, arg2, arg3) => arg1.size() + arg2.size() + arg3.size() + 3,
            Self::Substr(arg1, arg2, arg3) => arg1.size() + arg2.size() + arg3.size() + 3,
            Self::Add(arg1, arg2) => arg1.size() + arg2.size() + 2,
            Self::Sub(arg1, arg2) => arg1.size() + arg2.size() + 2,
            Self::Len(arg) => arg.size() + 1,
            Self::At(arg1, arg2) => arg1.size() + arg2.size() + 2,
            Self::ToStr(arg) => arg.size() + 1,
            Self::ToInt(arg) => arg.size() + 1,
            Self::IndexOf(arg1, arg2, arg3) => arg1.size() + arg2.size() + arg3.size() + 3,
            Self::PrefixOf(arg1, arg2) => arg1.size() + arg2.size() + 2,
            Self::SuffixOf(arg1, arg2) => arg1.size() + arg2.size() + 2,
            Self::Contains(arg1, arg2) => arg1.size() + arg2.size() + 2,
        }
    }
}
