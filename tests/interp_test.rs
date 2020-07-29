use absynthe::*;
use std::collections::HashMap;

#[test]
fn interp_int() {
    let env = HashMap::new();
    let prog = Expr::Const(Value::Int(5));

    assert_eq!(Value::Int(5), interp(&prog, &env));
}

#[test]
fn interp_env() {
    let mut env = HashMap::new();
    env.insert("x".to_string(), Value::Str("foo".to_string()));
    let prog = Expr::Var("x".to_string());

    assert_eq!(Value::Str("foo".to_string()), interp(&prog, &env));
}

#[test]
fn interp_append() {
    let env = HashMap::new();
    let prog = Expr::Call(Box::new(
        Func::Append(Expr::Const(Value::Str("hello ".to_string())),
                     Expr::Const(Value::Str("world".to_string())))));

    assert_eq!(Value::Str("hello world".to_string()), interp(&prog, &env));
}

#[test]
fn interp_err() {
    let env = HashMap::new();
    let prog = Expr::Call(Box::new(
        Func::Append(Expr::Const(Value::Str("hello ".to_string())),
                     Expr::Const(Value::Int(6)))));

    assert_eq!(Value::Error, interp(&prog, &env));
}

#[test]
fn interp_replace() {
    let env = HashMap::new();
    let prog = Expr::Call(Box::new(
        Func::Replace(Expr::Const(Value::Str("boop".to_string())),
                      Expr::Const(Value::Str("b".to_string())),
                      Expr::Const(Value::Str("p".to_string())))));

    assert_eq!(Value::Str("poop".to_string()), interp(&prog, &env));
}

#[test]
fn interp_substr() {
    let env = HashMap::new();
    let prog = Expr::Call(Box::new(
        Func::Substr(Expr::Const(Value::Str("Balloon".to_string())),
                      Expr::Const(Value::Int(1)),
                      Expr::Const(Value::Int(4)))));

    assert_eq!(Value::Str("all".to_string()), interp(&prog, &env));
}

#[test]
fn interp_add() {
    let env = HashMap::new();
    let prog = Expr::Call(Box::new(
        Func::Add(Expr::Const(Value::Int(1)),
                  Expr::Const(Value::Int(4)))));

    assert_eq!(Value::Int(5), interp(&prog, &env));
}

#[test]
fn interp_sub() {
    let env = HashMap::new();
    let prog = Expr::Call(Box::new(
        Func::Sub(Expr::Const(Value::Int(1)),
                  Expr::Const(Value::Int(4)))));

    assert_eq!(Value::Int(-3), interp(&prog, &env));
}

#[test]
fn interp_len() {
    let env = HashMap::new();
    let prog = Expr::Call(Box::new(
        Func::Len(Expr::Const(Value::Str("foo bar".to_string())))));

    assert_eq!(Value::Int(7), interp(&prog, &env));
}

#[test]
fn sygus_bikes() {
    let prog = Expr::Call(Box::new(
        Func::Substr(Expr::Var("arg0".to_string()),
                     Expr::Const(Value::Int(0)),
                     Expr::Call(Box::new(
                        Func::Sub(Expr::Call(Box::new(
                            Func::Len(Expr::Var("arg0".to_string())))),
                        Expr::Const(Value::Int(3))))))));

    let mut env = HashMap::new();
    env.insert("arg0".to_string(), Value::Str("Ducati100".to_string()));
    assert_eq!(Value::Str("Ducati".to_string()), interp(&prog, &env));

    env.insert("arg0".to_string(), Value::Str("Honda125".to_string()));
    assert_eq!(Value::Str("Honda".to_string()), interp(&prog, &env));

    env.insert("arg0".to_string(), Value::Str("Ducati250".to_string()));
    assert_eq!(Value::Str("Ducati".to_string()), interp(&prog, &env));

    env.insert("arg0".to_string(), Value::Str("Honda250".to_string()));
    assert_eq!(Value::Str("Honda".to_string()), interp(&prog, &env));

    env.insert("arg0".to_string(), Value::Str("Honda550".to_string()));
    assert_eq!(Value::Str("Honda".to_string()), interp(&prog, &env));

    env.insert("arg0".to_string(), Value::Str("Ducati125".to_string()));
    assert_eq!(Value::Str("Ducati".to_string()), interp(&prog, &env));
}
