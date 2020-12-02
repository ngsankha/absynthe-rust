use absynthe::concrete::*;
use absynthe::interpreter::*;
use absynthe::linear::*;
use absynthe::r#abstract::*;
use itertools::{Either, Itertools};
use std::collections::HashMap;

struct Context {
    conc_exprs: HashMap<u32, Vec<Expr<AbsStrVal, StrLenLat>>>,
    max_size: u32,
}

impl Context {
    fn new(consts: &Vec<StrVal>, env: &HashMap<String, AbsStrVal>) -> Context {
        let exprs0 = consts
            .into_iter()
            .map(|x| Expr::Const(AbsStrVal::from(x.clone())))
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

fn abs_append_inv(target: StrLenLat, lhs: StrLenLat) -> StrLenLat {
    target - lhs
}

fn abs_substr_inv(_target: StrLenLat, _start: StrVal, _end: StrVal) -> StrLenLat {
    StrLenLat::top()
}

fn gen_expr(
    ctx: &mut Context,
    start: Expr<AbsStrVal, StrLenLat>,
    consts: &Vec<StrVal>,
    env: &HashMap<String, AbsStrVal>,
) -> Vec<Expr<AbsStrVal, StrLenLat>> {
    let mut work_list = vec![start];

    while !work_list.is_empty() {
        let work_item = work_list.remove(0);
        let expanded = visit(ctx, work_item, consts, env);
        // TODO: insert testing code here
        let (concrete, with_holes): (Vec<_>, Vec<_>) =
            expanded.into_iter().partition_map(|x| match x.has_hole() {
                true => Either::Right(x),
                false => Either::Left(x),
            });
        if concrete.is_empty() {
            work_list.extend(with_holes.into_iter());
            // work_list = work_list
            //     .into_iter()
            //     .filter(|p| p.size() <= ctx.max_size)
            //     .collect();
            work_list.sort_by(|a, b| a.size().partial_cmp(&b.size()).unwrap());
        } else {
            return concrete;
        }
    }

    unreachable!()
}

fn gen_const(abs: &StrLenLat, consts: &Vec<StrVal>) -> Vec<Expr<AbsStrVal, StrLenLat>> {
    consts
        .iter()
        .map(|c| (c, c.abstraction()))
        .filter_map(|t| match t {
            (_, None) => None,
            (c, Some(a)) => {
                if a <= *abs {
                    Some(Expr::Const(AbsStrVal::from(c.clone())))
                } else {
                    None
                }
            }
        })
        .collect()
}

fn gen_var(abs: &StrLenLat, env: &HashMap<String, AbsStrVal>) -> Vec<Expr<AbsStrVal, StrLenLat>> {
    env.iter()
        .filter_map(|(k, v)| {
            if v <= &AbsStrVal::from(abs.clone()) {
                Some(Expr::Var(k.clone()))
            } else {
                None
            }
        })
        .collect()
}

fn gen_call(ctx: &mut Context, abs: &StrLenLat) -> Vec<Expr<AbsStrVal, StrLenLat>> {
    let mut expanded = Vec::new();
    expanded.push(gen_append(ctx, abs));
    expanded.push(gen_substr(ctx, abs));
    expanded
}

fn gen_append(ctx: &mut Context, abs: &StrLenLat) -> Expr<AbsStrVal, StrLenLat> {
    let lhs = Expr::ConcHole(0);
    let rhs = Expr::DepHole;
    let pending_expr = Func::Append(lhs, rhs);
    Expr::Hole(abs.clone(), Some(Box::new(pending_expr)))
}

fn gen_substr(ctx: &mut Context, abs: &StrLenLat) -> Expr<AbsStrVal, StrLenLat> {
    let arg1 = Expr::DepHole;
    let arg2 = Expr::ConcHole(0);
    let arg3 = Expr::ConcHole(0);
    let pending_expr = Func::Substr(arg1, arg2, arg3);
    Expr::Hole(abs.clone(), Some(Box::new(pending_expr)))
}

fn concretize(ctx: &mut Context, size: u32) -> Vec<Expr<AbsStrVal, StrLenLat>> {
    let cached = ctx.conc_exprs.get(&size);
    match cached {
        Some(res) => (*res).clone(),
        None => {
            // append(arg1, arg2)
            let appends: Vec<Expr<AbsStrVal, StrLenLat>> = (0..size)
                .combinations(2)
                .map(|args| {
                    let lhs_vec = concretize(ctx, args[0]);
                    let rhs_vec = concretize(ctx, args[1]);
                    lhs_vec
                        .into_iter()
                        .cartesian_product(rhs_vec.into_iter())
                        .map(|(lhs, rhs)| Expr::Call(Box::new(Func::Append(lhs, rhs))))
                })
                .flatten()
                .collect();

            // replace(arg1, arg2, arg3)
            let replaces: Vec<Expr<AbsStrVal, StrLenLat>> = (0..size)
                .combinations(3)
                .map(|args| {
                    let arg1_vec = concretize(ctx, args[0]);
                    let arg2_vec = concretize(ctx, args[1]);
                    let arg3_vec = concretize(ctx, args[2]);
                    arg1_vec
                        .into_iter()
                        .cartesian_product(arg2_vec.into_iter())
                        .cartesian_product(arg3_vec.into_iter())
                        .map(|((arg1, arg2), arg3)| {
                            Expr::Call(Box::new(Func::Replace(arg1, arg2, arg3)))
                        })
                })
                .flatten()
                .collect();

            // sub(arg1, arg2)
            let subs: Vec<Expr<AbsStrVal, StrLenLat>> = (0..size)
                .combinations(2)
                .map(|args| {
                    let arg1_vec = concretize(ctx, args[0]);
                    let arg2_vec = concretize(ctx, args[1]);
                    arg1_vec
                        .into_iter()
                        .cartesian_product(arg2_vec.into_iter())
                        .map(|(arg1, arg2)| Expr::Call(Box::new(Func::Sub(arg1, arg2))))
                })
                .flatten()
                .collect();
            // len(arg1)
            let lens: Vec<Expr<AbsStrVal, StrLenLat>> = (0..size)
                .combinations(1)
                .map(|args| {
                    let arg1_vec = concretize(ctx, args[0]);
                    arg1_vec
                        .into_iter()
                        .map(|arg1| Expr::Call(Box::new(Func::Len(arg1))))
                })
                .flatten()
                .collect();

            let all_exprs: Vec<Expr<AbsStrVal, StrLenLat>> = appends
                .into_iter()
                .chain(replaces.into_iter())
                .chain(subs.into_iter())
                .chain(lens.into_iter())
                .collect();
            ctx.conc_exprs.insert(size, all_exprs.clone());
            all_exprs
        }
    }
}

fn visit(
    ctx: &mut Context,
    expr: Expr<AbsStrVal, StrLenLat>,
    consts: &Vec<StrVal>,
    env: &HashMap<String, AbsStrVal>,
) -> Vec<Expr<AbsStrVal, StrLenLat>> {
    // println!("{:?}", expr);
    match expr {
        Expr::Hole(abs, None) => {
            let mut expanded = Vec::new();
            // constants
            expanded.extend(gen_const(&abs, &consts));

            // variables
            expanded.extend(gen_var(&abs, &env));

            // functions
            expanded.extend(gen_call(ctx, &abs));

            expanded
        }
        Expr::Hole(abs, Some(e)) => match *e {
            Func::Append(Expr::ConcHole(size), Expr::DepHole) => {
                let lhs_vec = concretize(ctx, size);
                let rhs_abs_vec = lhs_vec
                    .clone()
                    .into_iter()
                    .filter_map(|x| match StrLenInterp::eval(&x, &env).ok() {
                        Some(AbsStrVal::Abs(a_lhs)) => Some(abs_append_inv(abs.clone(), a_lhs)),
                        Some(AbsStrVal::Conc(a_lhs)) => {
                            a_lhs.abstraction().map(|v| abs_append_inv(abs.clone(), v))
                        }
                        _ => None,
                    })
                    .filter_map(|x| Some(Expr::<AbsStrVal, StrLenLat>::Hole(x, None)));
                lhs_vec
                    .into_iter()
                    .zip(rhs_abs_vec)
                    .map(|(arg1, arg2)| Expr::Call(Box::new(Func::Append(arg1, arg2))))
                    .chain(
                        // add the same expression with hole that can accomodate bigger arguments
                        vec![Expr::<AbsStrVal, StrLenLat>::Hole(
                            abs.clone(),
                            Some(Box::new(Func::Append(
                                Expr::ConcHole(size + 1),
                                Expr::DepHole,
                            ))),
                        )]
                        .into_iter(),
                    )
                    .collect()
            }
            _ => todo!(),
        },
        Expr::Call(f) => match *f {
            Func::Append(arg1, arg2) => {
                let arg1_expanded = visit(ctx, arg1, consts, env);
                let arg2_expanded = visit(ctx, arg2, consts, env);
                arg1_expanded
                    .into_iter()
                    .zip(arg2_expanded.into_iter())
                    .map(|(a1, a2)| Expr::Call(Box::new(Func::Append(a1, a2))))
                    .collect()
            }
            _ => todo!(),
        },
        _ => vec![expr],
    }
}

fn main() {
    let mut consts = Vec::new();
    consts.push(StrVal::from(3));

    let mut env = HashMap::new();
    env.insert("arg0".to_string(), AbsStrVal::from("x".to_string()));

    let target = LinearExpr::from("x".to_string()) - LinearExpr::from(3);

    let mut ctx = Context::new(&consts, &env);

    let start = Expr::Hole(StrLenLat::from(target), None);

    println!("{:?}", gen_expr(&mut ctx, start, &consts, &env));
}
// (substr arg0 0 (- (len arg) 3))
