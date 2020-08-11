use absynthe::r#abstract::*;
use absynthe::concrete::*;
use absynthe::interpreter::*;
use std::collections::HashMap;

#[test]
fn interp_int() {
    let env = HashMap::new();
    let prog = Expr::Const(StrVal::Int(5));

    assert_eq!(StrVal::Int(5), StrOpInterpreter::eval(&prog, &env));
}

#[test]
fn interp_env() {
    let mut env = HashMap::new();
    env.insert("x".to_string(), StrVal::Str("foo".to_string()));
    let prog = Expr::Var("x".to_string());

    assert_eq!(StrVal::Str("foo".to_string()), StrOpInterpreter::eval(&prog, &env));
}

#[test]
fn interp_append() {
    let env = HashMap::new();
    let prog = Expr::Call(Box::new(
        Func::Append(Expr::Const(StrVal::Str("hello ".to_string())),
                     Expr::Const(StrVal::Str("world".to_string())))));

    assert_eq!(StrVal::Str("hello world".to_string()), StrOpInterpreter::eval(&prog, &env));
}

#[test]
fn abs_interp_append_conc() {
    let env = HashMap::new();
    let prog = Expr::Call(Box::new(
        Func::Append(Expr::Const(AbsValue::Conc(StrVal::Str("hello ".to_string()))),
                     Expr::Const(AbsValue::Conc(StrVal::Str("world".to_string()))))));

    assert_eq!(AbsValue::Conc(StrVal::Str("hello world".to_string())), StrLenInterp::eval(&prog, &env));
}

#[test]
fn abs_interp_append_abs() {
    let env = HashMap::new();
    let prog = Expr::Call(Box::new(
        Func::Append(Expr::Const(AbsValue::Conc(StrVal::Str("hello ".to_string()))),
                     Expr::Const(AbsValue::Abs(StrLenLat::Len(5))))));

    assert_eq!(AbsValue::Abs(StrLenLat::Len(11)), StrLenInterp::eval(&prog, &env));
}

#[test]
fn abs_interp_append_err() {
    let env = HashMap::new();
    let prog = Expr::Call(Box::new(
        Func::Append(Expr::Const(AbsValue::Conc(StrVal::Int(6))),
                     Expr::Const(AbsValue::Abs(StrLenLat::Len(5))))));

    assert_eq!(AbsValue::Conc(StrVal::Error), StrLenInterp::eval(&prog, &env));
}

// #[test]
// fn interp_err() {
//     let env = HashMap::new();
//     let prog = Expr::Call(Box::new(
//         Func::Append(Expr::Const(Value::Str("hello ".to_string())),
//                      Expr::Const(Value::Int(6)))));

//     assert_eq!(Value::Error, interp(&prog, &env));
// }

// #[test]
// fn interp_replace() {
//     let env = HashMap::new();
//     let prog = Expr::Call(Box::new(
//         Func::Replace(Expr::Const(Value::Str("boop".to_string())),
//                       Expr::Const(Value::Str("b".to_string())),
//                       Expr::Const(Value::Str("p".to_string())))));

//     assert_eq!(Value::Str("poop".to_string()), interp(&prog, &env));
// }

// #[test]
// fn interp_substr() {
//     let env = HashMap::new();
//     let prog = Expr::Call(Box::new(
//         Func::Substr(Expr::Const(Value::Str("Balloon".to_string())),
//                       Expr::Const(Value::Int(1)),
//                       Expr::Const(Value::Int(4)))));

//     assert_eq!(Value::Str("all".to_string()), interp(&prog, &env));
// }

// #[test]
// fn interp_add() {
//     let env = HashMap::new();
//     let prog = Expr::Call(Box::new(
//         Func::Add(Expr::Const(Value::Int(1)),
//                   Expr::Const(Value::Int(4)))));

//     assert_eq!(Value::Int(5), interp(&prog, &env));
// }

// #[test]
// fn interp_sub() {
//     let env = HashMap::new();
//     let prog = Expr::Call(Box::new(
//         Func::Sub(Expr::Const(Value::Int(1)),
//                   Expr::Const(Value::Int(4)))));

//     assert_eq!(Value::Int(-3), interp(&prog, &env));
// }

// #[test]
// fn interp_len() {
//     let env = HashMap::new();
//     let prog = Expr::Call(Box::new(
//         Func::Len(Expr::Const(Value::Str("foo bar".to_string())))));

//     assert_eq!(Value::Int(7), interp(&prog, &env));
// }

#[test]
fn sygus_bikes() {
    let prog = Expr::Call(Box::new(
        Func::Substr(Expr::Var("arg0".to_string()),
                     Expr::Const(StrVal::Int(0)),
                     Expr::Call(Box::new(
                        Func::Sub(Expr::Call(Box::new(
                            Func::Len(Expr::Var("arg0".to_string())))),
                        Expr::Const(StrVal::Int(3))))))));

    let mut env = HashMap::new();
    env.insert("arg0".to_string(), StrVal::Str("Ducati100".to_string()));
    assert_eq!(StrVal::Str("Ducati".to_string()), StrOpInterpreter::eval(&prog, &env));

    env.insert("arg0".to_string(), StrVal::Str("Honda125".to_string()));
    assert_eq!(StrVal::Str("Honda".to_string()), StrOpInterpreter::eval(&prog, &env));

    env.insert("arg0".to_string(), StrVal::Str("Ducati250".to_string()));
    assert_eq!(StrVal::Str("Ducati".to_string()), StrOpInterpreter::eval(&prog, &env));

    env.insert("arg0".to_string(), StrVal::Str("Honda250".to_string()));
    assert_eq!(StrVal::Str("Honda".to_string()), StrOpInterpreter::eval(&prog, &env));

    env.insert("arg0".to_string(), StrVal::Str("Honda550".to_string()));
    assert_eq!(StrVal::Str("Honda".to_string()), StrOpInterpreter::eval(&prog, &env));

    env.insert("arg0".to_string(), StrVal::Str("Ducati125".to_string()));
    assert_eq!(StrVal::Str("Ducati".to_string()), StrOpInterpreter::eval(&prog, &env));
}

#[test]
fn abs_sygus_bikes() {
    let prog = Expr::Call(Box::new(
        Func::Substr(Expr::Var("arg0".to_string()),
                     Expr::Const(AbsValue::Conc(StrVal::Int(0))),
                     Expr::Call(Box::new(
                        Func::Sub(Expr::Call(Box::new(
                            Func::Len(Expr::Var("arg0".to_string())))),
                        Expr::Const(AbsValue::Conc(StrVal::Int(3)))))))));

    let mut env = HashMap::new();
    env.insert("arg0".to_string(), AbsValue::Abs(StrLenLat::Len(9)));
    assert_eq!(AbsValue::Abs(StrLenLat::Len(6)), StrLenInterp::eval(&prog, &env));

    env.insert("arg0".to_string(), AbsValue::Abs(StrLenLat::Len(8)));
    assert_eq!(AbsValue::Abs(StrLenLat::Len(5)), StrLenInterp::eval(&prog, &env));

    env.insert("arg0".to_string(), AbsValue::Abs(StrLenLat::Len(9)));
    assert_eq!(AbsValue::Abs(StrLenLat::Len(6)), StrLenInterp::eval(&prog, &env));

    env.insert("arg0".to_string(), AbsValue::Abs(StrLenLat::Len(8)));
    assert_eq!(AbsValue::Abs(StrLenLat::Len(5)), StrLenInterp::eval(&prog, &env));

    env.insert("arg0".to_string(), AbsValue::Abs(StrLenLat::Len(8)));
    assert_eq!(AbsValue::Abs(StrLenLat::Len(5)), StrLenInterp::eval(&prog, &env));

    env.insert("arg0".to_string(), AbsValue::Abs(StrLenLat::Len(9)));
    assert_eq!(AbsValue::Abs(StrLenLat::Len(6)), StrLenInterp::eval(&prog, &env));
}
