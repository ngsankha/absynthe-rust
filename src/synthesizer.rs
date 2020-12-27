use crate::concrete::StrVal;
use crate::environment::Environment;
use crate::interpreter::EvalResult;
use crate::interpreter::Evaluable;
use crate::interpreter::SynthesisVisitor;
use crate::r#abstract::StrValAbs;
use crate::strlenlat::StrLenLat;
use crate::syguslang::Expr;
use itertools::{Either, Itertools};
use std::collections::HashMap;

pub struct Context {
    conc_exprs: HashMap<u32, Vec<Expr<StrValAbs, StrLenLat>>>,
    max_size: u32,
}

impl Context {
    pub fn new(consts: &Vec<StrVal>, env: &Environment<StrValAbs>) -> Context {
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

pub struct Synthesizer;

impl Synthesizer {
    fn wrap_func(
        expr: Expr<StrValAbs, StrLenLat>,
    ) -> impl Fn(&[StrValAbs]) -> EvalResult<StrValAbs> {
        move |args: &[StrValAbs]| {
            let mut env = Environment::new();
            args.into_iter()
                .enumerate()
                .for_each(|(idx, val)| env.put(format!("arg{}", idx), val.clone()));
            expr.eval(&env)
        }
    }

    pub fn synthesize(
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

            let correct: Vec<Expr<StrValAbs, StrLenLat>> = concrete
                .into_iter()
                .filter(|p| {
                    let func = Self::wrap_func(p.clone());
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
}
