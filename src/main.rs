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

fn abs_substr_inv(_target: StrLenLat, _start: StrLenLat, _end: StrLenLat) -> StrLenLat {
    StrLenLat::top()
}

fn gen_expr(
    ctx: &mut Context,
    target: StrLenLat,
    consts: &Vec<StrVal>,
    env: &HashMap<String, AbsStrVal>,
) -> Vec<Expr<AbsStrVal, StrLenLat>> {
    let start = Expr::Hole(target.clone(), None);
    let mut work_list = vec![start];

    while !work_list.is_empty() {
        let work_item = work_list.remove(0);
        let expanded = visit(ctx, work_item, consts, env);

        let (concrete, with_holes): (Vec<_>, Vec<_>) =
            expanded.into_iter().partition_map(|x| match x.has_hole() {
                true => Either::Right(x),
                false => Either::Left(x),
            });

        // TODO: insert testing code here
        let correct: Vec<Expr<AbsStrVal, StrLenLat>> = concrete
            .into_iter()
            .filter(|p| {
                let mut env: HashMap<String, AbsStrVal> = HashMap::new();
                env.insert(
                    "arg0".to_string(),
                    AbsStrVal::Conc(StrVal::from("Ducati100".to_string())),
                );
                match StrLenInterp::eval(p, &env) {
                    Ok(AbsStrVal::Conc(StrVal::Str(v))) => v == "Ducati",
                    _ => false,
                }
            })
            .collect();

        if correct.is_empty() {
            work_list.extend(
                with_holes
                    .into_iter()
                    .filter(|p| match StrLenInterp::eval(p, env) {
                        Ok(AbsStrVal::Abs(v)) => v <= target,
                        _ => false,
                    }),
            );
            work_list = work_list
                .into_iter()
                .filter(|p| p.size() <= ctx.max_size)
                .collect();
            work_list.sort_by(|a, b| a.size().partial_cmp(&b.size()).unwrap());
        } else {
            return correct;
        }
    }

    panic!("no solutions found")
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

            // substr(arg1, arg2, arg3)
            let substrs: Vec<Expr<AbsStrVal, StrLenLat>> = (0..size)
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
                            Expr::Call(Box::new(Func::Substr(arg1, arg2, arg3)))
                        })
                })
                .flatten()
                .collect();

            // add(arg1, arg2)
            let adds: Vec<Expr<AbsStrVal, StrLenLat>> = (0..size)
                .combinations(2)
                .map(|args| {
                    let arg1_vec = concretize(ctx, args[0]);
                    let arg2_vec = concretize(ctx, args[1]);
                    arg1_vec
                        .into_iter()
                        .cartesian_product(arg2_vec.into_iter())
                        .map(|(arg1, arg2)| Expr::Call(Box::new(Func::Add(arg1, arg2))))
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
                .chain(substrs.into_iter())
                .chain(adds.into_iter())
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
            Func::Substr(Expr::DepHole, Expr::ConcHole(size1), Expr::ConcHole(size2)) => {
                let arg2_vec = concretize(ctx, size1);
                let arg3_vec = concretize(ctx, size2);
                arg2_vec
                    .into_iter()
                    .cartesian_product(arg3_vec.into_iter())
                    .map(|(arg2, arg3)| {
                        // the inverse always returns top, so no need to evaluate arg2 and arg3
                        Expr::Call(Box::new(Func::Substr(
                            Expr::Hole(
                                abs_substr_inv(abs.clone(), StrLenLat::top(), StrLenLat::top()),
                                None,
                            ),
                            arg2,
                            arg3,
                        )))
                    })
                    .chain(
                        vec![
                            Expr::<AbsStrVal, StrLenLat>::Hole(
                                abs.clone(),
                                Some(Box::new(Func::Substr(
                                    Expr::DepHole,
                                    Expr::ConcHole(size1 + 1),
                                    Expr::ConcHole(size2),
                                ))),
                            ),
                            Expr::<AbsStrVal, StrLenLat>::Hole(
                                abs.clone(),
                                Some(Box::new(Func::Substr(
                                    Expr::DepHole,
                                    Expr::ConcHole(size1),
                                    Expr::ConcHole(size2 + 1),
                                ))),
                            ),
                        ]
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
            Func::Replace(arg1, arg2, arg3) => {
                let arg1_expanded = visit(ctx, arg1, consts, env);
                let arg2_expanded = visit(ctx, arg2, consts, env);
                let arg3_expanded = visit(ctx, arg3, consts, env);
                arg1_expanded
                    .into_iter()
                    .zip(arg2_expanded.into_iter())
                    .zip(arg3_expanded.into_iter())
                    .map(|((a1, a2), a3)| Expr::Call(Box::new(Func::Replace(a1, a2, a3))))
                    .collect()
            }
            Func::Substr(arg1, arg2, arg3) => {
                let arg1_expanded = visit(ctx, arg1, consts, env);
                let arg2_expanded = visit(ctx, arg2, consts, env);
                let arg3_expanded = visit(ctx, arg3, consts, env);
                arg1_expanded
                    .into_iter()
                    .zip(arg2_expanded.into_iter())
                    .zip(arg3_expanded.into_iter())
                    .map(|((a1, a2), a3)| Expr::Call(Box::new(Func::Substr(a1, a2, a3))))
                    .collect()
            }
            Func::Len(arg1) => {
                let arg1_expanded = visit(ctx, arg1, consts, env);
                arg1_expanded
                    .into_iter()
                    .map(|a1| Expr::Call(Box::new(Func::Len(a1))))
                    .collect()
            }
            Func::Add(arg1, arg2) => {
                let arg1_expanded = visit(ctx, arg1, consts, env);
                let arg2_expanded = visit(ctx, arg2, consts, env);
                arg1_expanded
                    .into_iter()
                    .zip(arg2_expanded.into_iter())
                    .map(|(a1, a2)| Expr::Call(Box::new(Func::Add(a1, a2))))
                    .collect()
            }
            Func::Sub(arg1, arg2) => {
                let arg1_expanded = visit(ctx, arg1, consts, env);
                let arg2_expanded = visit(ctx, arg2, consts, env);
                arg1_expanded
                    .into_iter()
                    .zip(arg2_expanded.into_iter())
                    .map(|(a1, a2)| Expr::Call(Box::new(Func::Sub(a1, a2))))
                    .collect()
            }
        },
        _ => vec![expr],
    }
}

fn main() {
    let mut consts = Vec::new();
    consts.push(StrVal::from(0));
    consts.push(StrVal::from(3));

    let mut env = HashMap::new();
    env.insert(
        "arg0".to_string(),
        AbsStrVal::Abs(StrLenLat::from("x".to_string())),
    );

    let target = LinearExpr::from("x".to_string()) - LinearExpr::from(3);

    let mut ctx = Context::new(&consts, &env);

    // let correct: Expr<AbsStrVal, StrLenLat> = Expr::Call(Box::new(Func::Substr(
    //     Expr::Var("arg0".to_string()),
    //     Expr::Const(AbsStrVal::Conc(StrVal::Int(LinearExpr::from(0)))),
    //     Expr::Call(Box::new(Func::Sub(
    //         Expr::Call(Box::new(Func::Len(Expr::Var("arg0".to_string())))),
    //         Expr::Const(AbsStrVal::Conc(StrVal::Int(LinearExpr::from(3)))),
    //     ))),
    // )));

    // env.insert(
    //     "arg0".to_string(),
    //     AbsStrVal::Conc(StrVal::Str("Ducati100".to_string())),
    // );
    // println!("{:?}", StrLenInterp::eval(&correct, &env).unwrap());

    // Call(Substr(
    //     Var("arg0"),
    //     Const(Conc(Int(LinearExpr { c: 0, terms: {} }))),
    //     Call(Sub(
    //         Const(Conc(Int(LinearExpr { c: 0, terms: {} }))),
    //         Call(Sub(Const(Conc(Int(LinearExpr { c: 3, terms: {} }))), Call(Len(Var("arg0")))))))))

    println!(
        "{}",
        gen_expr(&mut ctx, StrLenLat::from(target), &consts, &env)[0]
    );
}
// (substr arg0 0 (- 0 (- 3 (len arg0)))
