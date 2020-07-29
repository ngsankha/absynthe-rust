use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Str(String),
    Int(i64),
    Error
}

pub enum Func {
    Append(Expr, Expr),
    Replace(Expr, Expr, Expr),
    Substr(Expr, Expr, Expr),
    Add(Expr, Expr),
    Sub(Expr, Expr),
    Len(Expr)
}

pub enum Expr {
    Const(Value),
    Var(String),
    Call(Box<Func>)
}

pub fn interp(expr: &Expr, env: &HashMap<String, Value>) -> Value {
    match expr {
        Expr::Const(v) => v.clone(),
        Expr::Var(x) => match env.get(x) {
            Some(val) => val.clone(),
            None => Value::Error
        }
        Expr::Call(call) => eval_call(call, env)
    }
}

fn eval_call(call: &Func, env: &HashMap<String, Value>) -> Value {
    match call {
        Func::Append(s1, s2) => str_append(interp(s1, env), interp(s2, env)),
        Func::Replace(s1, s2, s3) => str_replace(interp(s1, env), interp(s2, env), interp(s3, env)),
        Func::Substr(s1, s2, s3) => str_substr(interp(s1, env), interp(s2, env), interp(s3, env)),
        Func::Add(i, j) => int_add(interp(i, env), interp(j, env)),
        Func::Sub(i, j) => int_sub(interp(i, env), interp(j, env)),
        Func::Len(s) => str_len(interp(s, env))
    }
}

fn str_append(v1: Value, v2: Value) -> Value {
    match v1 {
        Value::Str(s1) => match v2 {
            Value::Str(s2) => Value::Str(s1 + &s2),
            _ => Value::Error
        },
        _ => Value::Error
    }
}

fn str_replace(v1: Value, v2: Value, v3: Value) -> Value {
    match v1 {
        Value::Str(s1) => match v2 {
            Value::Str(s2) => match v3 {
                Value::Str(s3) => Value::Str(s1.replace(&s2, &s3)),
                _ => Value::Error
            }
            _ => Value::Error
        },
        _ => Value::Error
    }
}

fn str_substr(v1: Value, v2: Value, v3: Value) -> Value {
    match v1 {
        Value::Str(s1) => match v2 {
            Value::Int(start) => match v3 {
                Value::Int(end) => Value::Str(s1.chars().skip(start as usize).take((end - start) as usize).collect()),
                _ => Value::Error
            }
            _ => Value::Error
        },
        _ => Value::Error
    }
}

fn int_add(v1: Value, v2: Value) -> Value {
    match v1 {
        Value::Int(i) => match v2 {
            Value::Int(j) => Value::Int(i + j),
            _ => Value::Error
        },
        _ => Value::Error
    }
}

fn int_sub(v1: Value, v2: Value) -> Value {
    match v1 {
        Value::Int(i) => match v2 {
            Value::Int(j) => Value::Int(i - j),
            _ => Value::Error
        },
        _ => Value::Error
    }
}

fn str_len(v: Value) -> Value {
    match v {
        Value::Str(s) => Value::Int(s.chars().count() as i64),
        _ => Value::Error
    }
}
