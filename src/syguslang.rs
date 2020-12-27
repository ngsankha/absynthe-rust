use crate::values::{Lattice, Value};
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Func<T: Value, U: Lattice> {
    Append(Expr<T, U>, Expr<T, U>),
    Replace(Expr<T, U>, Expr<T, U>, Expr<T, U>),
    Substr(Expr<T, U>, Expr<T, U>, Expr<T, U>),
    Add(Expr<T, U>, Expr<T, U>),
    Sub(Expr<T, U>, Expr<T, U>),
    Len(Expr<T, U>),
    At(Expr<T, U>, Expr<T, U>),
    ToStr(Expr<T, U>),
    ToInt(Expr<T, U>),
    IndexOf(Expr<T, U>, Expr<T, U>, Expr<T, U>),
    PrefixOf(Expr<T, U>, Expr<T, U>),
    SuffixOf(Expr<T, U>, Expr<T, U>),
    Contains(Expr<T, U>, Expr<T, U>),
}

#[derive(Debug, Clone)]
pub enum Expr<T: Value, U: Lattice> {
    Const(T),
    Var(String),
    Call(Box<Func<T, U>>),
    If(Box<Expr<T, U>>, Box<Expr<T, U>>, Box<Expr<T, U>>),
    Hole(U, Option<Box<Func<T, U>>>),
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
        (match self {
            Self::Append(arg1, arg2) => arg1.size() + arg2.size(),
            Self::Replace(arg1, arg2, arg3) => arg1.size() + arg2.size() + arg3.size(),
            Self::Substr(arg1, arg2, arg3) => arg1.size() + arg2.size() + arg3.size(),
            Self::Add(arg1, arg2) => arg1.size() + arg2.size(),
            Self::Sub(arg1, arg2) => arg1.size() + arg2.size(),
            Self::Len(arg) => arg.size(),
            Self::At(arg1, arg2) => arg1.size() + arg2.size(),
            Self::ToStr(arg) => arg.size(),
            Self::ToInt(arg) => arg.size(),
            Self::IndexOf(arg1, arg2, arg3) => arg1.size() + arg2.size() + arg3.size(),
            Self::PrefixOf(arg1, arg2) => arg1.size() + arg2.size(),
            Self::SuffixOf(arg1, arg2) => arg1.size() + arg2.size(),
            Self::Contains(arg1, arg2) => arg1.size() + arg2.size(),
        }) + 1
    }
}
