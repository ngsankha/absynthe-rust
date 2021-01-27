use crate::concrete::StrVal;
use crate::environment::Environment;
use crate::interpreter::EvalResult;
use crate::interpreter::Evaluable;
use crate::interpreter::SynthesisVisitor;
use crate::syguslang::Expr;
use crate::syguslang::Term;
use crate::types::{TypeLattice, TypeValues};
use crate::values::Lattice;
use crate::values::Value;
use itertools::{Either, Itertools};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::rc::Rc;

type LatticeValues = TypeValues;
type MyLattice = TypeLattice;

pub struct Context {
    conc_exprs: HashMap<u32, Vec<Term<LatticeValues, MyLattice>>>,
    max_size: u32,
}

impl Context {
    pub fn new(consts: &Vec<StrVal>, env: &Environment<LatticeValues>) -> Context {
        let exprs0 = consts
            .into_iter()
            .map(|x| Rc::new(Expr::Const(LatticeValues::from_concrete(x.clone()))))
            .chain(env.keys().map(|x| Rc::new(Expr::Var(x.clone()))))
            .collect();
        let mut expr_map = HashMap::new();
        expr_map.insert(0, exprs0);
        Context {
            conc_exprs: expr_map,
            max_size: 10,
        }
    }
}

impl<T: Value + Eq, U: Lattice + Eq> Ord for Expr<T, U> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.size().cmp(&self.size())
    }
}

impl<T: Value + Eq, U: Lattice + Eq> PartialOrd for Expr<T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Synthesizer;

impl Synthesizer {
    fn wrap_func(expr: Term<StrVal, MyLattice>) -> impl Fn(&[StrVal]) -> EvalResult<StrVal> {
        move |args: &[StrVal]| {
            let mut env = Environment::new();
            args.into_iter()
                .enumerate()
                .for_each(|(idx, val)| env.put(format!("arg{}", idx), val.clone()));
            expr.eval(&env)
        }
    }

    pub fn synthesize(
        ctx: &mut Context,
        target: MyLattice,
        env: &Environment<LatticeValues>,
        test: Box<dyn Fn(Box<dyn Fn(&[StrVal]) -> EvalResult<StrVal>>) -> bool>,
    ) -> Vec<Term<LatticeValues, MyLattice>> {
        let start = Rc::new(Expr::Hole(target.clone(), None));
        let mut work_list = BinaryHeap::new();
        work_list.push(start);

        while !work_list.is_empty() {
            let work_item = work_list.pop().unwrap();
            // println!("{}", work_item);
            let expanded = work_item.visit(env, &mut ctx.conc_exprs);

            let (concrete, with_holes): (Vec<_>, Vec<_>) =
                expanded.into_iter().partition_map(|x| match x.has_hole() {
                    true => Either::Right(x),
                    false => Either::Left(x),
                });

            let correct: Vec<Term<LatticeValues, MyLattice>> = concrete
                .into_iter()
                .filter(|p| {
                    let func = Self::wrap_func(Rc::new(Expr::from(p.clone())));
                    test(Box::new(func))
                })
                .collect();

            if correct.is_empty() {
                with_holes
                    .into_iter()
                    .filter(|p| p.size() <= ctx.max_size)
                    .for_each(|p| work_list.push(p));
            } else {
                return correct;
            }
        }

        panic!("no solutions found")
    }
}
