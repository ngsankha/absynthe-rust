use crate::interpreter::*;
use crate::linear::*;
// use crate::r#abstract::*;
use crate::environment::Environment;
use std::fmt;
use std::fmt::Display;

pub type EvalResult<T> = Result<T, &'static str>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StrVal {
    Str(String),
    Int(LinearExpr),
    Bool(bool),
}

impl Value for StrVal {}

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

impl Display for StrVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StrVal::Str(s) => write!(f, "\"{}\"", s),
            StrVal::Int(i) => write!(f, "{}", i),
            StrVal::Bool(b) => write!(f, "{}", b),
        }
    }
}

impl<T: Value + Display, U: Lattice> Display for Expr<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Const(c) => write!(f, "{}", c),
            Self::Var(var) => write!(f, "{}", var),
            Self::Call(func) => write!(f, "{}", func),
            Self::If(cond, then, otherwise) => write!(f, "(if {} {} {})", cond, then, otherwise),
            _ => unreachable!(),
        }
    }
}

impl<T: Value + Display, U: Lattice> Display for Func<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Append(arg1, arg2) => write!(f, "(append {} {})", arg1, arg2),
            Self::Replace(arg1, arg2, arg3) => write!(f, "(replace {} {} {})", arg1, arg2, arg3),
            Self::Substr(arg1, arg2, arg3) => write!(f, "(append {} {} {})", arg1, arg2, arg3),
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
            Self::ConcHole(_) => true,
            Self::DepHole => true,
            Self::Const(_) => false,
            Self::Var(_) => false,
            Self::Call(f) => match &*(*f) {
                Func::Append(arg1, arg2) => arg1.has_hole() || arg2.has_hole(),
                Func::Replace(arg1, arg2, arg3) => {
                    arg1.has_hole() || arg2.has_hole() || arg3.has_hole()
                }
                Func::Substr(arg1, arg2, arg3) => {
                    arg1.has_hole() || arg2.has_hole() || arg3.has_hole()
                }
                Func::Add(arg1, arg2) => arg1.has_hole() || arg2.has_hole(),
                Func::Sub(arg1, arg2) => arg1.has_hole() || arg2.has_hole(),
                Func::Len(arg) => arg.has_hole(),
                Func::At(arg1, arg2) => arg1.has_hole() || arg2.has_hole(),
                Func::ToStr(arg) => arg.has_hole(),
                Func::ToInt(arg) => arg.has_hole(),
                Func::IndexOf(arg1, arg2, arg3) => {
                    arg1.has_hole() || arg2.has_hole() || arg3.has_hole()
                }
                Func::PrefixOf(arg1, arg2) => arg1.has_hole() || arg2.has_hole(),
                Func::SuffixOf(arg1, arg2) => arg1.has_hole() || arg2.has_hole(),
                Func::Contains(arg1, arg2) => arg1.has_hole() || arg2.has_hole(),
            },
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

impl From<String> for StrVal {
    fn from(item: String) -> Self {
        StrVal::Str(item)
    }
}

impl From<i32> for StrVal {
    fn from(item: i32) -> Self {
        StrVal::Int(LinearExpr::from(item))
    }
}

impl<U: Lattice> Evaluable<StrVal, U> for Expr<StrVal, U> {
    fn eval(&self, env: &Environment<StrVal>) -> EvalResult<StrVal> {
        match self {
            Self::Const(v) => Ok(v.clone()),
            Self::Var(x) => env
                .get(x.clone())
                .map(|v| v.clone())
                .ok_or_else(|| "variable not found"),
            Self::Call(call) => call.eval(env),
            Self::If(cond, then, otherwise) => {
                let cond_evaled = cond.eval(env);
                let then_evaled = then.eval(env);
                let otherwise_evaled = otherwise.eval(env);
                match (cond_evaled, then_evaled, otherwise_evaled) {
                    (Ok(c), Ok(t), Ok(o)) => Self::eval_if(c, t, o),
                    _ => Err("if: invalid argument"),
                }
            }
            _ => Err("holes cannot be processed"),
        }
    }
}

impl<U: Lattice> Expr<StrVal, U> {
    fn eval_if(cond: StrVal, then: StrVal, otherwise: StrVal) -> EvalResult<StrVal> {
        match (cond, then, otherwise) {
            (StrVal::Bool(b), t, o) => Ok(if b { t } else { o }),
            _ => Err("invalid types"),
        }
    }
}

impl<U: Lattice> Evaluable<StrVal, U> for Func<StrVal, U> {
    fn eval(&self, env: &Environment<StrVal>) -> EvalResult<StrVal> {
        match self {
            Self::Append(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env);
                let arg2_evaled = arg2.eval(env);
                match (arg1_evaled, arg2_evaled) {
                    (Ok(a1), Ok(a2)) => Self::str_append(a1, a2),
                    _ => Err("append: invalid argument"),
                }
            }
            Self::Replace(arg1, arg2, arg3) => {
                let arg1_evaled = arg1.eval(env);
                let arg2_evaled = arg2.eval(env);
                let arg3_evaled = arg3.eval(env);
                match (arg1_evaled, arg2_evaled, arg3_evaled) {
                    (Ok(a1), Ok(a2), Ok(a3)) => Self::str_replace(a1, a2, a3),
                    _ => Err("replace: invalid argument"),
                }
            }
            Self::Substr(arg1, arg2, arg3) => {
                let arg1_evaled = arg1.eval(env);
                let arg2_evaled = arg2.eval(env);
                let arg3_evaled = arg3.eval(env);
                match (arg1_evaled, arg2_evaled, arg3_evaled) {
                    (Ok(a1), Ok(a2), Ok(a3)) => Self::str_substr(a1, a2, a3),
                    _ => Err("substr: invalid argument"),
                }
            }
            Self::Add(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env);
                let arg2_evaled = arg2.eval(env);
                match (arg1_evaled, arg2_evaled) {
                    (Ok(a1), Ok(a2)) => Self::int_add(a1, a2),
                    _ => Err("add: invalid argument"),
                }
            }
            Self::Sub(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env);
                let arg2_evaled = arg2.eval(env);
                match (arg1_evaled, arg2_evaled) {
                    (Ok(a1), Ok(a2)) => Self::int_sub(a1, a2),
                    _ => Err("sub: invalid argument"),
                }
            }
            Self::Len(arg) => {
                let arg_evaled = arg.eval(env);
                match arg_evaled {
                    Ok(a) => Self::str_len(a),
                    _ => Err("len: invalid argument"),
                }
            }
            Self::At(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env);
                let arg2_evaled = arg2.eval(env);
                match (arg1_evaled, arg2_evaled) {
                    (Ok(a1), Ok(a2)) => Self::str_at(a1, a2),
                    _ => Err("at: invalid argument"),
                }
            }
            Self::ToStr(arg) => {
                let arg_evaled = arg.eval(env);
                match arg_evaled {
                    Ok(a) => Self::int_to_str(a),
                    _ => Err("tostr: invalid argument"),
                }
            }
            Self::ToInt(arg) => {
                let arg_evaled = arg.eval(env);
                match arg_evaled {
                    Ok(a) => Self::str_to_int(a),
                    _ => Err("toint: invalid argument"),
                }
            }
            Self::IndexOf(arg1, arg2, arg3) => {
                let arg1_evaled = arg1.eval(env);
                let arg2_evaled = arg2.eval(env);
                let arg3_evaled = arg3.eval(env);
                match (arg1_evaled, arg2_evaled, arg3_evaled) {
                    (Ok(a1), Ok(a2), Ok(a3)) => Self::str_indexof(a1, a2, a3),
                    _ => Err("indexof: invalid argument"),
                }
            }
            Self::PrefixOf(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env);
                let arg2_evaled = arg2.eval(env);
                match (arg1_evaled, arg2_evaled) {
                    (Ok(a1), Ok(a2)) => Self::str_prefixof(a1, a2),
                    _ => Err("at: invalid argument"),
                }
            }
            Self::SuffixOf(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env);
                let arg2_evaled = arg2.eval(env);
                match (arg1_evaled, arg2_evaled) {
                    (Ok(a1), Ok(a2)) => Self::str_suffixof(a1, a2),
                    _ => Err("at: invalid argument"),
                }
            }
            Self::Contains(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env);
                let arg2_evaled = arg2.eval(env);
                match (arg1_evaled, arg2_evaled) {
                    (Ok(a1), Ok(a2)) => Self::str_contains(a1, a2),
                    _ => Err("at: invalid argument"),
                }
            }
        }
    }
}

impl<U: Lattice> Func<StrVal, U> {
    fn str_append(v1: StrVal, v2: StrVal) -> EvalResult<StrVal> {
        match (v1, v2) {
            (StrVal::Str(s1), StrVal::Str(s2)) => Ok(StrVal::Str(s1 + &s2)),
            _ => Err("invalid types"),
        }
    }

    fn str_replace(v1: StrVal, v2: StrVal, v3: StrVal) -> EvalResult<StrVal> {
        match (v1, v2, v3) {
            (StrVal::Str(s1), StrVal::Str(s2), StrVal::Str(s3)) => {
                Ok(StrVal::Str(s1.replace(&s2, &s3)))
            }
            _ => Err("invalid types"),
        }
    }

    fn str_substr(v1: StrVal, v2: StrVal, v3: StrVal) -> EvalResult<StrVal> {
        match (v1, v2, v3) {
            (StrVal::Str(s1), StrVal::Int(start), StrVal::Int(end)) => {
                let start_int = start.as_const();
                let end_int = end.as_const();
                match (start_int, end_int) {
                    (Some(s), Some(e)) => Ok(StrVal::Str(
                        s1.chars().skip(s as usize).take(e as usize).collect(),
                    )),
                    _ => Err("arguments not constant"),
                }
            }
            _ => Err("invalid types"),
        }
    }

    fn int_add(v1: StrVal, v2: StrVal) -> EvalResult<StrVal> {
        match (v1, v2) {
            (StrVal::Int(i), StrVal::Int(j)) => Ok(StrVal::Int(i + j)),
            _ => Err("invalid types"),
        }
    }

    fn int_sub(v1: StrVal, v2: StrVal) -> EvalResult<StrVal> {
        match (v1, v2) {
            (StrVal::Int(i), StrVal::Int(j)) => Ok(StrVal::Int(i - j)),
            _ => Err("invalid types"),
        }
    }

    fn str_len(v: StrVal) -> EvalResult<StrVal> {
        match v {
            StrVal::Str(s) => Ok(StrVal::Int(LinearExpr::from(s.chars().count() as i32))),
            _ => Err("invalid types"),
        }
    }

    fn str_at(v1: StrVal, v2: StrVal) -> EvalResult<StrVal> {
        match (v1, v2) {
            (StrVal::Str(s), StrVal::Int(i)) => i
                .as_const()
                .and_then(|i| s.chars().nth(i as usize))
                .ok_or_else(|| "invalid index")
                .map(|c| StrVal::Str(c.to_string())),
            _ => Err("invalid types"),
        }
    }

    fn str_to_int(v: StrVal) -> EvalResult<StrVal> {
        match v {
            StrVal::Int(i) => i
                .as_const()
                .ok_or_else(|| "expected constant")
                .map(|s| StrVal::Str(s.to_string())),
            _ => Err("invalid types"),
        }
    }

    fn int_to_str(v: StrVal) -> EvalResult<StrVal> {
        match v {
            StrVal::Str(s) => s
                .parse::<i32>()
                .map_err(|_| "not a number")
                .map(|v| StrVal::Int(LinearExpr::from(v))),
            _ => Err("invalid types"),
        }
    }

    fn str_indexof(v1: StrVal, v2: StrVal, v3: StrVal) -> EvalResult<StrVal> {
        match (v1, v2, v3) {
            (StrVal::Str(recv), StrVal::Str(pat), StrVal::Int(start)) => {
                let at = start.as_const();
                match at {
                    Some(s) => Ok(StrVal::Int(LinearExpr::from(
                        recv[(s as usize)..]
                            .find(&pat)
                            .map_or_else(|| -1, |i| s + (i as i32)),
                    ))),
                    None => Err("expected constant index"),
                }
            }
            _ => Err("invalid types"),
        }
    }

    fn str_prefixof(v1: StrVal, v2: StrVal) -> EvalResult<StrVal> {
        match (v1, v2) {
            (StrVal::Str(s1), StrVal::Str(s2)) => Ok(StrVal::Bool(s1.starts_with(&s2))),
            _ => Err("invalid types"),
        }
    }

    fn str_suffixof(v1: StrVal, v2: StrVal) -> EvalResult<StrVal> {
        match (v1, v2) {
            (StrVal::Str(s1), StrVal::Str(s2)) => Ok(StrVal::Bool(s1.ends_with(&s2))),
            _ => Err("invalid types"),
        }
    }

    fn str_contains(v1: StrVal, v2: StrVal) -> EvalResult<StrVal> {
        match (v1, v2) {
            (StrVal::Str(s1), StrVal::Str(s2)) => Ok(StrVal::Bool(s1.contains(&s2))),
            _ => Err("invalid types"),
        }
    }
}
