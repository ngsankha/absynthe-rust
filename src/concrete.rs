use crate::environment::Environment;
use crate::interpreter::{EvalResult, Evaluable};
use crate::linear::LinearExpr;
use crate::syguslang::{Expr, Func};
use crate::values::{Lattice, Value};
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StrVal {
    Str(String),
    Int(LinearExpr),
    Bool(bool),
}

impl Value for StrVal {
    fn is_abstract(&self) -> bool {
        false
    }
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

impl From<LinearExpr> for StrVal {
    fn from(item: LinearExpr) -> Self {
        StrVal::Int(item)
    }
}

impl<U: Lattice> Evaluable<StrVal> for Expr<StrVal, U> {
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
            _ => unreachable!(),
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

impl<U: Lattice> Evaluable<StrVal> for Func<StrVal, U> {
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
