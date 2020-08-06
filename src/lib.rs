use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub enum StrLenLat {
    Top,
    Len(usize),
    Bot
}

trait Lattice: PartialOrd + PartialEq {
    // fn meet(&self, other: &Self) -> Self;
    fn join(&self, other: &Self) -> Self;
}

impl PartialOrd for StrLenLat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            StrLenLat::Top => match other {
                StrLenLat::Top => Some(Ordering::Equal),
                _ => Some(Ordering::Greater)
            },
            StrLenLat::Len(x) => match other {
                StrLenLat::Top => Some(Ordering::Less),
                StrLenLat::Len(y) => x.partial_cmp(y),
                StrLenLat::Bot => Some(Ordering::Greater)
            },
            StrLenLat::Bot => match other {
                StrLenLat::Bot => Some(Ordering::Equal),
                _ => Some(Ordering::Less)
            }
        }
    }
}

impl Lattice for StrLenLat {
    fn join(&self, other: &Self) -> Self {
        match self {
            StrLenLat::Top => self.clone(),
            StrLenLat::Len(v1) => match other {
                StrLenLat::Top => other.clone(),
                StrLenLat::Len(v2) => if v1 > v2 {
                    StrLenLat::Len(*v1)
                } else {
                    StrLenLat::Len(*v2)
                },
                StrLenLat::Bot => self.clone()
            },
            StrLenLat::Bot => other.clone()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AbsValue<T, U> {
    Abs(T),
    Conc(U)
}

#[derive(Debug, Clone, PartialEq)]
pub enum StrVal {
    Str(String),
    Int(i64),
    Error
}

pub enum Func<T> {
    Append(Expr<T>, Expr<T>),
    // Replace(Expr<T>, Expr<T>, Expr<T>),
    // Substr(Expr<T>, Expr<T>, Expr<T>),
    // Add(Expr<T>, Expr<T>),
    // Sub(Expr<T>, Expr<T>),
    // Len(Expr<T>)
}

pub enum Expr<T> {
    Const(T),
    Var(String),
    Call(Box<Func<T>>)
}

pub trait Value: Clone {
    fn error() -> Self;
}

trait Abstractable<T> {
    fn abstraction(&self) -> T;
}

impl Abstractable<AbsStrVal> for StrVal {
    fn abstraction(&self) -> AbsStrVal {
        match self {
            StrVal::Str(s) => AbsValue::Abs(StrLenLat::Len(s.chars().count())),
            _ => AbsValue::Conc(self.clone())
        }
    }
}

impl Value for StrVal {
    fn error() -> Self {
        StrVal::Error
    }
}

type AbsStrVal = AbsValue<StrLenLat, StrVal>;

impl Value for AbsStrVal {
    fn error() -> Self {
        AbsValue::Conc(StrVal::Error)
    }
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

pub struct StrOpInterpreter;

pub struct StrLenInterp;

impl Interpreter<AbsStrVal> for StrLenInterp {
    fn eval_call(expr: &Func<AbsStrVal>, env: &HashMap<String, AbsStrVal>) -> AbsStrVal {
        match expr {
            Func::Append(s1, s2) => Self::abs_str_append(Self::eval(s1, env), Self::eval(s2, env)),
            // Func::Replace(s1, s2, s3) => str_replace(interp(s1, env), interp(s2, env), interp(s3, env)),
            // Func::Substr(s1, s2, s3) => str_substr(interp(s1, env), interp(s2, env), interp(s3, env)),
            // Func::Add(i, j) => int_add(interp(i, env), interp(j, env)),
            // Func::Sub(i, j) => int_sub(interp(i, env), interp(j, env)),
            // Func::Len(s) => str_len(interp(s, env))
        }
    }
}

impl StrLenInterp {
    fn abs_str_append(v1: AbsStrVal, v2: AbsStrVal) -> AbsStrVal {
        match (v1.clone(), v2.clone()) {
            (AbsValue::Abs(s1), AbsValue::Abs(s2)) => match (s1, s2) {
                (StrLenLat::Top, _) => AbsValue::Abs(StrLenLat::Top),
                (_, StrLenLat::Top) => AbsValue::Abs(StrLenLat::Top),
                (StrLenLat::Len(l1), StrLenLat::Len(l2)) => AbsValue::Abs(StrLenLat::Len(l1 + l2)),
                (StrLenLat::Len(l1), StrLenLat::Bot) => AbsValue::Abs(StrLenLat::Len(l1)),
                (StrLenLat::Bot, StrLenLat::Len(l2)) => AbsValue::Abs(StrLenLat::Len(l2)),
                (_, _) => AbsValue::Abs(StrLenLat::Bot)
            },
            (AbsValue::Conc(c1), AbsValue::Conc(c2)) => match (c1.clone(), c2.clone()) {
                (StrVal::Str(s1), StrVal::Str(s2)) => AbsValue::Conc(StrVal::Str(s1 + &s2)),
                (_, _) => AbsValue::Conc(StrVal::Error)
            },
            (AbsValue::Conc(c), AbsValue::Abs(_)) => Self::abs_str_append(c.abstraction(), v2),
            (AbsValue::Abs(_), AbsValue::Conc(c)) => Self::abs_str_append(v1, c.abstraction()),
        }
    }
}

impl Interpreter<StrVal> for StrOpInterpreter {
    fn eval_call(call: &Func<StrVal>, env: &HashMap<String, StrVal>) -> StrVal {
        match call {
            Func::Append(s1, s2) => Self::str_append(Self::eval(s1, env), Self::eval(s2, env)),
            // Func::Replace(s1, s2, s3) => str_replace(interp(s1, env), interp(s2, env), interp(s3, env)),
            // Func::Substr(s1, s2, s3) => str_substr(interp(s1, env), interp(s2, env), interp(s3, env)),
            // Func::Add(i, j) => int_add(interp(i, env), interp(j, env)),
            // Func::Sub(i, j) => int_sub(interp(i, env), interp(j, env)),
            // Func::Len(s) => str_len(interp(s, env))
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
}

// pub fn interp<T: Value, Clone>(expr: &Expr<T>, env: &HashMap<String, T>) -> T {
//     match expr {
//         Expr::Const(v) => *v.clone(),
//         Expr::Var(x) => match env.get(x) {
//             Some(val) => *val.clone(),
//             None => T::error()
//         }
//         Expr::Call(call) => eval_call(call, env)
//     }
// }

// fn eval_call<T>(call: &Func<T>, env: &HashMap<String, T>) -> T {
//     match call {
//         Func::Append(s1, s2) => str_append(interp(s1, env), interp(s2, env)),
//         // Func::Replace(s1, s2, s3) => str_replace(interp(s1, env), interp(s2, env), interp(s3, env)),
//         // Func::Substr(s1, s2, s3) => str_substr(interp(s1, env), interp(s2, env), interp(s3, env)),
//         // Func::Add(i, j) => int_add(interp(i, env), interp(j, env)),
//         // Func::Sub(i, j) => int_sub(interp(i, env), interp(j, env)),
//         // Func::Len(s) => str_len(interp(s, env))
//     }
// }

// fn str_append<T>(v1: StrVal, v2: StrVal) -> StrVal {
//     match (v1, v2) {
//         (StrVal::Str(s1), StrVal::Str(s2)) => StrVal::Str(s1 + &s2),
//         _ => StrVal::error()
//     }
// }

// fn abs_str_append(v1: StringLenLattice, v2: StringLenLattice) -> StringLenLattice {
//     match (v1, v2) {
//         (StringLenLattice::Top, _) => StringLenLattice::Top,
//         (_, StringLenLattice::Top) => StringLenLattice::Top,
//         (StringLenLattice::Len(l1), StringLenLattice::Len(l2)) => StringLenLattice::Len(l1 + l2),
//         (StringLenLattice::Len(_), StringLenLattice::Bot) => StringLenLattice::Bot,
//         (StringLenLattice::Bot, StringLenLattice::Len(_)) => StringLenLattice::Bot,
//         (StringLenLattice::Bot, StringLenLattice::Bot) => StringLenLattice::Bot
//     }
// }

// fn str_replace(v1: StrVal, v2: StrVal, v3: StrVal) -> StrVal {
//     match (v1, v2, v3) {
//         (StrVal::Str(s1), StrVal::Str(s2), StrVal::Str(s3)) => StrVal::Str(s1.replace(&s2, &s3)),
//         _ => StrVal::error()
//     }
// }

// fn abs_str_replace(v1: StringLenLattice, v2: StringLenLattice, v3: StringLenLattice) -> StringLenLattice {
//     // match (v1, v2, v3) {
//     //     (StrVal::Str(s1), StrVal::Str(s2), StrVal::Str(s3)) => StrVal::Str(s1.replace(&s2, &s3)),
//     //     _ => StrVal::error()
//     // }
//     StringLenLattice::Top
// }

// fn str_substr(v1: StrVal, v2: StrVal, v3: StrVal) -> StrVal {
//     match (v1, v2, v3) {
//         (StrVal::Str(s1), StrVal::Int(start), StrVal::Int(end)) => StrVal::Str(s1.chars().skip(start as usize).take((end - start) as usize).collect()),
//         _ => StrVal::error()
//     }
// }

// fn int_add(v1: StrVal, v2: StrVal) -> StrVal {
//     match (v1, v2) {
//         (StrVal::Int(i), StrVal::Int(j)) => StrVal::Int(i + j),
//         _ => StrVal::error()
//     }
// }

// fn int_sub(v1: StrVal, v2: StrVal) -> StrVal {
//     match (v1, v2) {
//         (StrVal::Int(i), StrVal::Int(j)) => StrVal::Int(i - j),
//         _ => StrVal::error()
//     }
// }

// fn str_len(v: StrVal) -> StrVal {
//     match v {
//         StrVal::Str(s) => StrVal::Int(s.chars().count() as i64),
//         _ => StrVal::error()
//     }
// }
