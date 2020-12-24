use absynthe::concrete::StrVal;
use absynthe::environment::Environment;
use absynthe::interpreter::{Evaluable, SynthesisVisitor};
use absynthe::linear::LinearExpr;
use absynthe::r#abstract::StrValAbs;
use absynthe::strlenlat::StrLenLat;
use absynthe::syguslang::Expr;
use itertools::{Either, Itertools};
use std::collections::HashMap;

struct Context {
    conc_exprs: HashMap<u32, Vec<Expr<StrValAbs, StrLenLat>>>,
    max_size: u32,
}

impl Context {
    fn new(consts: &Vec<StrVal>, env: &Environment<StrValAbs>) -> Context {
        let exprs0 = consts
            .into_iter()
            .map(|x| Expr::Const(StrValAbs::from_concrete(x.clone())))
            .chain(env.keys().map(|x| Expr::Var(x.clone())))
            .collect();
        let mut expr_map = HashMap::new();
        expr_map.insert(0, exprs0);
        Context {
            conc_exprs: expr_map,
            max_size: 5,
        }
    }
}

fn synthesize(
    ctx: &mut Context,
    target: StrLenLat,
    env: &Environment<StrValAbs>,
) -> Vec<Expr<StrValAbs, StrLenLat>> {
    let start = Expr::Hole(target.clone(), None);
    let mut work_list = vec![start];

    while !work_list.is_empty() {
        let work_item = work_list.remove(0);
        let expanded = work_item.visit(env, &mut ctx.conc_exprs);

        let (concrete, with_holes): (Vec<_>, Vec<_>) =
            expanded.into_iter().partition_map(|x| match x.has_hole() {
                true => Either::Right(x),
                false => Either::Left(x),
            });

        // TODO: insert testing code here
        let correct: Vec<Expr<StrValAbs, StrLenLat>> = concrete
            .into_iter()
            .filter(|p| {
                let mut env = Environment::new();
                env.put(
                    "arg0".to_string(),
                    StrValAbs::Conc(StrVal::from("Ducati100".to_string())),
                );
                match p.eval(&env) {
                    Ok(StrValAbs::Conc(StrVal::Str(v))) => v == "Ducati",
                    _ => false,
                }
            })
            .collect();

        if correct.is_empty() {
            work_list = work_list
                .into_iter()
                .chain(with_holes.into_iter())
                .filter(|p| p.size() <= ctx.max_size)
                .collect();
            work_list.sort_by(|a, b| a.size().partial_cmp(&b.size()).unwrap());
        } else {
            return correct;
        }
    }

    panic!("no solutions found")
}

fn main() {
    let mut consts = Vec::new();
    consts.push(StrVal::from(0));
    consts.push(StrVal::from(3));

    let mut env = Environment::new();
    env.put(
        "arg0".to_string(),
        StrValAbs::Abs(StrLenLat::from("x".to_string())),
    );

    let target = LinearExpr::from("x".to_string()) - LinearExpr::from(3);

    let mut ctx = Context::new(&consts, &env);

    // let correct: Expr<StrValAbs, StrLenLat> = Expr::Call(Box::new(Func::Substr(
    //     Expr::Var("arg0".to_string()),
    //     Expr::Const(StrValAbs::Conc(StrVal::Int(LinearExpr::from(0)))),
    //     Expr::Call(Box::new(Func::Sub(
    //         Expr::Call(Box::new(Func::Len(Expr::Var("arg0".to_string())))),
    //         Expr::Const(StrValAbs::Conc(StrVal::Int(LinearExpr::from(3)))),
    //     ))),
    // )));

    // env.put(
    //     "arg0".to_string(),
    //     StrValAbs::Conc(StrVal::Str("Ducati100".to_string())),
    // );

    // println!("{:?}", correct.eval(&env));

    println!("{}", synthesize(&mut ctx, StrLenLat::from(target), &env)[0]);
}
// (substr arg0 0 (- 0 (- 3 (len arg0)))
