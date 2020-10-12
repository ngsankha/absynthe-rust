use std::collections::{HashSet, HashMap};
use absynthe::r#abstract::*;
use absynthe::concrete::*;
use absynthe::interpreter::*;
use absynthe::linear::*;

#[derive(Debug)]
enum Lang<T: Lattice, U: Value> {
    Hole(T),
    Conc(Expr<U>)
}

fn abs_append_inv(target: StrLenLat, lhs: StrLenLat) -> StrLenLat {
    target - lhs
}

fn abs_substr_inv(_target: StrLenLat, _start: StrVal, _end: StrVal) -> StrLenLat {
    StrLenLat::top()
}

fn gen_expr(abs: StrLenLat, consts: Vec<StrVal>, env: HashMap<String, StrLenLat>) -> Vec<Lang<StrLenLat, StrVal>> {
    let mut expanded = Vec::new();

    // constants
    expanded.extend(gen_const(&abs, consts));

    // variables
    expanded.extend(gen_var(&abs, env));

    // functions
    expanded.extend(gen_call(&abs, env));

    expanded
}

fn gen_const(abs: &StrLenLat, consts: Vec<StrVal>) -> Vec<Lang<StrLenLat, StrVal>> {
    consts.iter()
    .map(|c| (c, c.abstraction()))
    .filter_map(|t| match t {
        (_, None) => None,
        (c, Some(a)) => if a <= *abs {
            Some(Lang::Conc(Expr::Const(c.clone())))
        } else {
            None
        }
    }).collect()
}

fn gen_var(abs: &StrLenLat, env: HashMap<String, StrLenLat>) -> Vec<Lang<StrLenLat, StrVal>> {
    env.iter()
    .filter_map(|(k, v)|
    if v <= abs {
        Some(Lang::Conc(Expr::Var(k.clone())))
    } else {
        None
    }).collect()
}

fn gen_call(abs: &StrLenLat)

fn main() {
    // let mut consts = HashSet::new();
    // consts.insert(StrVal::Int(0));
    // consts.insert(StrVal::Int(3));

    // let mut env = HashMap::new();
    // env.insert("arg0".to_string(), LinearExpr::from("x".to_string()));

    // let target = LinearExpr::from("x".to_string()) - LinearExpr::from(3);

    // let start = Lang::Hole(StrLenLat::Len(target));

    // let prog = Expr::Call(Box::new(
    //     Func::Substr(Expr::Var("arg0".to_string()),
    //                  Expr::Const(StrVal::Int(0)),
    //                  Expr::Call(Box::new(
    //                     Func::Sub(Expr::Call(Box::new(
    //                         Func::Len(Expr::Var("arg0".to_string())))),
    //                     Expr::Const(StrVal::Int(3))))))));

    let mut consts = Vec::new();
    consts.push(StrVal::from("sankha".to_string()));
    consts.push(StrVal::from("sohini".to_string()));
    consts.push(StrVal::from("hello".to_string()));
    consts.push(StrVal::from("balloon".to_string()));

    let mut env = HashMap::new();
    env.insert("arg0".to_string(), StrLenLat::from(6));

    let target = LinearExpr::from(LinearExpr::from(6));

    // let start = Lang::Hole(StrLenLat::Len(target));

    println!("{:?}", gen_expr(StrLenLat::from(target), consts, env));
}
