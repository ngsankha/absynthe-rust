use absynthe::concrete::StrVal;
use absynthe::environment::Environment;
use absynthe::interpreter::EvalResult;
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

fn wrap_func(expr: Expr<StrValAbs, StrLenLat>) -> impl Fn(&[StrValAbs]) -> EvalResult<StrValAbs> {
    move |args: &[StrValAbs]| {
        let mut env = Environment::new();
        args.into_iter()
            .enumerate()
            .for_each(|(idx, val)| env.put(format!("arg{}", idx), val.clone()));
        expr.eval(&env)
    }
}

fn synthesize(
    ctx: &mut Context,
    target: StrLenLat,
    env: &Environment<StrValAbs>,
    test: Box<dyn Fn(Box<dyn Fn(&[StrValAbs]) -> EvalResult<StrValAbs>>) -> bool>,
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
        // for c in concrete.clone() {
        //     println!("{}", c);
        // }

        let correct: Vec<Expr<StrValAbs, StrLenLat>> = concrete
            .into_iter()
            .filter(|p| {
                let func = wrap_func(p.clone());
                test(Box::new(func))
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
    let consts = vec![
        StrVal::from(0),
        StrVal::from(1),
        StrVal::from(2),
        StrVal::from(3),
        // StrVal::from(4),
        // StrVal::from(5),
        StrVal::from(" ".to_string()),
    ];

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

    let test = |f: Box<dyn Fn(&[StrValAbs]) -> EvalResult<StrValAbs>>| {
        f(&[StrValAbs::Conc(StrVal::from("Ducati100".to_string()))])
            == Ok(StrValAbs::Conc(StrVal::from("Ducati".to_string())))
            && f(&[StrValAbs::Conc(StrVal::from("Honda125".to_string()))])
                == Ok(StrValAbs::Conc(StrVal::from("Honda".to_string())))
            && f(&[StrValAbs::Conc(StrVal::from("Ducati250".to_string()))])
                == Ok(StrValAbs::Conc(StrVal::from("Ducati".to_string())))
            && f(&[StrValAbs::Conc(StrVal::from("Honda250".to_string()))])
                == Ok(StrValAbs::Conc(StrVal::from("Honda".to_string())))
            && f(&[StrValAbs::Conc(StrVal::from("Honda550".to_string()))])
                == Ok(StrValAbs::Conc(StrVal::from("Honda".to_string())))
            && f(&[StrValAbs::Conc(StrVal::from("Ducati125".to_string()))])
                == Ok(StrValAbs::Conc(StrVal::from("Ducati".to_string())))
    };

    println!(
        "{}",
        synthesize(&mut ctx, StrLenLat::from(target), &env, Box::new(test))[0]
    );
}
// (substr arg0 0 (- 0 (- 3 (len arg0)))
