use crate::concrete::StrVal;
use crate::environment::Environment;
use crate::interpreter::{ConcretizedSynth, EvalResult, Evaluable, SynthesisVisitor};
use crate::strlenlat::StrLenLat;
use crate::syguslang::{Expr, Func};
use crate::values::{Lattice, MixedValue};
use itertools::Itertools;
use std::collections::HashMap;
use std::convert::TryFrom;

pub type StrValAbs = MixedValue<StrVal, StrLenLat>;

impl Evaluable<StrValAbs> for Expr<StrValAbs, StrLenLat> {
    fn eval(&self, env: &Environment<StrValAbs>) -> EvalResult<StrValAbs> {
        match self {
            Self::Const(v) => Ok(v.clone()),
            Self::Var(x) => env
                .get(x.clone())
                .map(|v| v.clone())
                .ok_or_else(|| "variable not found"),
            Self::Call(call) => call.eval(env),
            Self::If(cond, then, otherwise) => unimplemented!(),
            Self::Hole(abs, _) => Ok(StrValAbs::from_abstract(abs.clone())),
            _ => unreachable!(),
        }
    }
}

impl Evaluable<StrValAbs> for Func<StrValAbs, StrLenLat> {
    fn eval(&self, env: &Environment<StrValAbs>) -> EvalResult<StrValAbs> {
        match self {
            Self::Append(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env);
                let arg2_evaled = arg2.eval(env);
                match (arg1_evaled, arg2_evaled) {
                    (Ok(StrValAbs::Conc(c1)), Ok(StrValAbs::Conc(c2))) => {
                        Func::Append::<StrVal, StrLenLat>(Expr::Const(c1), Expr::Const(c2))
                            .eval(&Environment::new())
                            .map(|v| StrValAbs::from_concrete(v))
                    }
                    (Ok(a1), Ok(a2)) => Self::str_append(a1, a2),
                    _ => Err("append: invalid argument"),
                }
            }
            Self::Replace(arg1, arg2, arg3) => {
                let arg1_evaled = arg1.eval(env);
                let arg2_evaled = arg2.eval(env);
                let arg3_evaled = arg3.eval(env);
                match (arg1_evaled, arg2_evaled, arg3_evaled) {
                    (Ok(StrValAbs::Conc(c1)), Ok(StrValAbs::Conc(c2)), Ok(StrValAbs::Conc(c3))) => {
                        Func::Replace::<StrVal, StrLenLat>(
                            Expr::Const(c1),
                            Expr::Const(c2),
                            Expr::Const(c3),
                        )
                        .eval(&Environment::new())
                        .map(|v| StrValAbs::from_concrete(v))
                    }
                    (Ok(a1), Ok(a2), Ok(a3)) => Self::str_replace(a1, a2, a3),
                    _ => Err("replace: invalid argument"),
                }
            }
            Self::Substr(arg1, arg2, arg3) => {
                let arg1_evaled = arg1.eval(env);
                let arg2_evaled = arg2.eval(env);
                let arg3_evaled = arg3.eval(env);
                match (arg1_evaled, arg2_evaled, arg3_evaled) {
                    (Ok(StrValAbs::Conc(c1)), Ok(StrValAbs::Conc(c2)), Ok(StrValAbs::Conc(c3))) => {
                        Func::Substr::<StrVal, StrLenLat>(
                            Expr::Const(c1),
                            Expr::Const(c2),
                            Expr::Const(c3),
                        )
                        .eval(&Environment::new())
                        .map(|v| StrValAbs::from_concrete(v))
                    }
                    (Ok(a1), Ok(a2), Ok(a3)) => Self::str_substr(a1, a2, a3),
                    _ => Err("substr: invalid argument"),
                }
            }
            Self::Add(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env);
                let arg2_evaled = arg2.eval(env);
                match (arg1_evaled, arg2_evaled) {
                    (Ok(StrValAbs::Conc(c1)), Ok(StrValAbs::Conc(c2))) => {
                        Func::Add::<StrVal, StrLenLat>(Expr::Const(c1), Expr::Const(c2))
                            .eval(&Environment::new())
                            .map(|v| StrValAbs::from_concrete(v))
                    }
                    (Ok(_), Ok(_)) => Err("invalid types"),
                    _ => Err("substr: invalid argument"),
                }
            }
            Self::Sub(arg1, arg2) => {
                let arg1_evaled = arg1.eval(env);
                let arg2_evaled = arg2.eval(env);
                match (arg1_evaled, arg2_evaled) {
                    (Ok(StrValAbs::Conc(c1)), Ok(StrValAbs::Conc(c2))) => {
                        Func::Sub::<StrVal, StrLenLat>(Expr::Const(c1), Expr::Const(c2))
                            .eval(&Environment::new())
                            .map(|v| StrValAbs::from_concrete(v))
                    }
                    (Ok(_), Ok(_)) => Err("invalid types"),
                    _ => Err("substr: invalid argument"),
                }
            }
            Self::Len(arg) => {
                let arg_evaled = arg.eval(env);
                match arg_evaled {
                    Ok(StrValAbs::Conc(c)) => Func::Len::<StrVal, StrLenLat>(Expr::Const(c))
                        .eval(&Environment::new())
                        .map(|v| StrValAbs::from_concrete(v)),
                    Ok(a) => Self::str_len(a),
                    _ => Err("substr: invalid argument"),
                }
            }
            Self::At(arg1, arg2) => unimplemented!(),
            Self::ToStr(arg) => unimplemented!(),
            Self::ToInt(arg) => unimplemented!(),
            Self::IndexOf(arg1, arg2, arg3) => unimplemented!(),
            Self::PrefixOf(arg1, arg2) => unimplemented!(),
            Self::SuffixOf(arg1, arg2) => unimplemented!(),
            Self::Contains(arg1, arg2) => unimplemented!(),
        }
    }
}

impl Func<StrValAbs, StrLenLat> {
    fn str_append(arg1: StrValAbs, arg2: StrValAbs) -> EvalResult<StrValAbs> {
        let absarg1 = StrLenLat::try_from(arg1);
        let absarg2 = StrLenLat::try_from(arg2);
        match (absarg1, absarg2) {
            (Ok(a1), Ok(a2)) => Ok(StrValAbs::from_abstract(a1 + a2)),
            _ => Err("invalid types"),
        }
    }

    fn str_replace(arg1: StrValAbs, arg2: StrValAbs, arg3: StrValAbs) -> EvalResult<StrValAbs> {
        let absarg1 = StrLenLat::try_from(arg1);
        let absarg2 = StrLenLat::try_from(arg2);
        let absarg3 = StrLenLat::try_from(arg3);
        match (absarg1, absarg2, absarg3) {
            (Ok(_), Ok(_), Ok(_)) => Ok(StrValAbs::from_abstract(StrLenLat::top())),
            _ => Err("invalid types"),
        }
    }

    fn str_substr(arg1: StrValAbs, arg2: StrValAbs, arg3: StrValAbs) -> EvalResult<StrValAbs> {
        let absarg1 = StrLenLat::try_from(arg1);
        match (absarg1, arg2, arg3) {
            (Ok(s), StrValAbs::Conc(StrVal::Int(start)), StrValAbs::Conc(StrVal::Int(end))) => {
                match s {
                    StrLenLat::Top => Ok(StrValAbs::from_abstract(StrLenLat::top())),
                    StrLenLat::Len(l) => {
                        if l >= start && end <= l && start <= end {
                            Ok(StrValAbs::from_abstract(StrLenLat::from(end - start)))
                        } else {
                            Err("substring index mismatch")
                        }
                    }
                    StrLenLat::Bot => Ok(StrValAbs::from_abstract(StrLenLat::bot())),
                }
            }
            _ => Err("invalid types"),
        }
    }

    fn str_len(arg: StrValAbs) -> EvalResult<StrValAbs> {
        let absarg = StrLenLat::try_from(arg);
        match absarg {
            Ok(StrLenLat::Len(l)) => Ok(StrValAbs::from_concrete(StrVal::from(l))),
            _ => Err("cannot lift ⊤/⊥ to concrete int"),
        }
    }
}

impl ConcretizedSynth<StrValAbs, StrLenLat> for Expr<StrValAbs, StrLenLat> {
    fn concretize(
        env: &Environment<StrValAbs>,
        size: u32,
        cache: &mut HashMap<u32, Vec<Expr<StrValAbs, StrLenLat>>>,
    ) -> Vec<Expr<StrValAbs, StrLenLat>> {
        let cached = cache.get(&size);

        match cached {
            Some(res) => (*res).clone(),
            None => {
                let generated: Vec<Expr<StrValAbs, StrLenLat>> =
                    Self::concretize_appends(env, size, cache)
                        .into_iter()
                        .chain(Self::concretize_replaces(env, size, cache).into_iter())
                        .chain(Self::concretize_substrs(env, size, cache).into_iter())
                        .chain(Self::concretize_adds(env, size, cache).into_iter())
                        .chain(Self::concretize_subs(env, size, cache).into_iter())
                        .chain(Self::concretize_lens(env, size, cache).into_iter())
                        .filter(|p| match p.eval(env) {
                            Err(_) => false,
                            _ => true,
                        })
                        .collect();
                cache.insert(size, generated.clone());
                generated
            }
        }
    }
}

impl Expr<StrValAbs, StrLenLat> {
    fn concretize_appends(
        env: &Environment<StrValAbs>,
        size: u32,
        cache: &mut HashMap<u32, Vec<Expr<StrValAbs, StrLenLat>>>,
    ) -> Vec<Expr<StrValAbs, StrLenLat>> {
        (0..size)
            .permutations(2)
            .map(|args| {
                let arg1_vec = Self::concretize(env, args[0], cache);
                let arg2_vec = Self::concretize(env, args[1], cache);

                arg1_vec
                    .into_iter()
                    .cartesian_product(arg2_vec.into_iter())
                    .map(|(arg1, arg2)| Expr::Call(Box::new(Func::Append(arg1, arg2))))
            })
            .flatten()
            .collect()
    }

    fn concretize_replaces(
        env: &Environment<StrValAbs>,
        size: u32,
        cache: &mut HashMap<u32, Vec<Expr<StrValAbs, StrLenLat>>>,
    ) -> Vec<Expr<StrValAbs, StrLenLat>> {
        (0..size)
            .permutations(3)
            .map(|args| {
                let arg1_vec = Self::concretize(env, args[0], cache);
                let arg2_vec = Self::concretize(env, args[1], cache);
                let arg3_vec = Self::concretize(env, args[2], cache);

                arg1_vec
                    .into_iter()
                    .cartesian_product(arg2_vec.into_iter())
                    .cartesian_product(arg3_vec.into_iter())
                    .map(|((arg1, arg2), arg3)| {
                        Expr::Call(Box::new(Func::Replace(arg1, arg2, arg3)))
                    })
            })
            .flatten()
            .collect()
    }

    fn concretize_substrs(
        env: &Environment<StrValAbs>,
        size: u32,
        cache: &mut HashMap<u32, Vec<Expr<StrValAbs, StrLenLat>>>,
    ) -> Vec<Expr<StrValAbs, StrLenLat>> {
        (0..size)
            .permutations(3)
            .map(|args| {
                let arg1_vec = Self::concretize(env, args[0], cache);
                let arg2_vec = Self::concretize(env, args[1], cache);
                let arg3_vec = Self::concretize(env, args[2], cache);

                arg1_vec
                    .into_iter()
                    .cartesian_product(arg2_vec.into_iter())
                    .cartesian_product(arg3_vec.into_iter())
                    .map(|((arg1, arg2), arg3)| {
                        Expr::Call(Box::new(Func::Substr(arg1, arg2, arg3)))
                    })
            })
            .flatten()
            .collect()
    }

    fn concretize_adds(
        env: &Environment<StrValAbs>,
        size: u32,
        cache: &mut HashMap<u32, Vec<Expr<StrValAbs, StrLenLat>>>,
    ) -> Vec<Expr<StrValAbs, StrLenLat>> {
        (0..size)
            .permutations(2)
            .map(|args| {
                let arg1_vec = Self::concretize(env, args[0], cache);
                let arg2_vec = Self::concretize(env, args[1], cache);

                arg1_vec
                    .into_iter()
                    .cartesian_product(arg2_vec.into_iter())
                    .map(|(arg1, arg2)| Expr::Call(Box::new(Func::Add(arg1, arg2))))
            })
            .flatten()
            .collect()
    }

    fn concretize_subs(
        env: &Environment<StrValAbs>,
        size: u32,
        cache: &mut HashMap<u32, Vec<Expr<StrValAbs, StrLenLat>>>,
    ) -> Vec<Expr<StrValAbs, StrLenLat>> {
        (0..size)
            .permutations(2)
            .map(|args| {
                let arg1_vec = Self::concretize(env, args[0], cache);
                let arg2_vec = Self::concretize(env, args[1], cache);

                arg1_vec
                    .into_iter()
                    .cartesian_product(arg2_vec.into_iter())
                    .map(|(arg1, arg2)| Expr::Call(Box::new(Func::Sub(arg1, arg2))))
            })
            .flatten()
            .collect()
    }

    fn concretize_lens(
        env: &Environment<StrValAbs>,
        size: u32,
        cache: &mut HashMap<u32, Vec<Expr<StrValAbs, StrLenLat>>>,
    ) -> Vec<Expr<StrValAbs, StrLenLat>> {
        (0..size)
            .permutations(2)
            .map(|args| {
                let arg1_vec = Self::concretize(env, args[0], cache);

                arg1_vec
                    .into_iter()
                    .map(|arg1| Expr::Call(Box::new(Func::Len(arg1))))
            })
            .flatten()
            .collect()
    }
}

impl SynthesisVisitor<StrValAbs, StrLenLat> for Expr<StrValAbs, StrLenLat> {
    fn visit(
        &self,
        env: &Environment<StrValAbs>,
        cache: &mut HashMap<u32, Vec<Expr<StrValAbs, StrLenLat>>>,
    ) -> Vec<Expr<StrValAbs, StrLenLat>> {
        // println!("{}", self);
        match self {
            Expr::If(cond, then, otherwise) => cond
                .visit(env, cache)
                .into_iter()
                .zip(then.visit(env, cache).into_iter())
                .zip(otherwise.visit(env, cache).into_iter())
                .map(|((a1, a2), a3)| Expr::If(Box::new(a1), Box::new(a2), Box::new(a3)))
                .collect(),
            Expr::Call(f) => f.visit(env, cache),
            Expr::Hole(abs, expr) => Self::visit_hole(abs, expr, env, cache),
            Expr::ConcHole(_) => unreachable!(),
            Expr::DepHole => unreachable!(),
            Expr::Const(c) => vec![Expr::Const(c.clone())],
            Expr::Var(v) => vec![Expr::Var(v.clone())],
        }
    }
}

impl Expr<StrValAbs, StrLenLat> {
    fn visit_hole(
        target: &StrLenLat,
        expr: &Option<Box<Func<StrValAbs, StrLenLat>>>,
        env: &Environment<StrValAbs>,
        cache: &mut HashMap<u32, Vec<Expr<StrValAbs, StrLenLat>>>,
    ) -> Vec<Expr<StrValAbs, StrLenLat>> {
        match expr {
            None => cache
                .get(&0)
                .map_or(vec![], |v| v.clone())
                .into_iter()
                .filter(|p| match p.eval(env) {
                    Ok(v) => v <= StrValAbs::from_abstract(target.clone()),
                    Err(_) => false,
                })
                .chain(
                    Self::gen_call()
                        .into_iter()
                        .map(|f| Expr::Hole(target.clone(), Some(Box::new(f)))),
                )
                .collect(),
            // the following match should have all the functions returned by gen_call
            Some(partial_expr) => match **partial_expr {
                Func::Append(Expr::ConcHole(size), Expr::DepHole) => {
                    Self::concretize(env, size, cache)
                        .into_iter()
                        .filter_map(|conc| {
                            Self::str_append_inv(target, &conc, env).map(|lat| {
                                Expr::Call(Box::new(Func::Append(conc, Expr::Hole(lat, None))))
                            })
                        })
                        .chain(vec![Expr::Hole(
                            target.clone(),
                            Some(Box::new(Func::Append(
                                Expr::ConcHole(size + 1),
                                Expr::DepHole,
                            ))),
                        )])
                        .collect()
                }
                Func::Substr(Expr::DepHole, Expr::ConcHole(size1), Expr::ConcHole(size2)) => {
                    // println!("{}", partial_expr);
                    // println!("here ==> {} {}", size1, size2);
                    Self::concretize(env, size1, cache)
                        .into_iter()
                        .cartesian_product(Self::concretize(env, size2, cache).into_iter())
                        .filter_map(|(conc1, conc2)| {
                            Self::str_substr_inv(target, &conc1, &conc2, env).map(|lat| {
                                Expr::Call(Box::new(Func::Substr(
                                    Expr::Hole(lat, None),
                                    conc1,
                                    conc2,
                                )))
                            })
                        })
                        .chain(vec![
                            Expr::Hole(
                                target.clone(),
                                Some(Box::new(Func::Substr(
                                    Expr::DepHole,
                                    Expr::ConcHole(size1 + 1),
                                    Expr::ConcHole(size2),
                                ))),
                            ),
                            Expr::Hole(
                                target.clone(),
                                Some(Box::new(Func::Substr(
                                    Expr::DepHole,
                                    Expr::ConcHole(size1),
                                    Expr::ConcHole(size2 + 1),
                                ))),
                            ),
                        ])
                        .collect()
                }
                _ => unreachable!(),
            },
        }
    }

    fn str_append_inv(
        target: &StrLenLat,
        arg1: &Expr<StrValAbs, StrLenLat>,
        env: &Environment<StrValAbs>,
    ) -> Option<StrLenLat> {
        match arg1.eval(env) {
            Ok(v) => match v {
                StrValAbs::Abs(lat) => Some(target.clone() - lat),
                StrValAbs::Conc(c) => StrLenLat::try_from(c).ok().map(|lat| target.clone() - lat),
            },
            _ => None,
        }
    }

    fn str_substr_inv(
        _target: &StrLenLat,
        arg2: &Expr<StrValAbs, StrLenLat>,
        arg3: &Expr<StrValAbs, StrLenLat>,
        env: &Environment<StrValAbs>,
    ) -> Option<StrLenLat> {
        match (arg2.eval(env), arg3.eval(env)) {
            (Ok(StrValAbs::Conc(StrVal::Int(_))), Ok(StrValAbs::Conc(StrVal::Int(_)))) => {
                Some(StrLenLat::top())
            }
            _ => None,
        }
    }

    fn gen_call() -> Vec<Func<StrValAbs, StrLenLat>> {
        vec![
            Func::Append(Expr::ConcHole(0), Expr::DepHole),
            Func::Substr(Expr::DepHole, Expr::ConcHole(0), Expr::ConcHole(0)),
        ]
    }
}

impl SynthesisVisitor<StrValAbs, StrLenLat> for Func<StrValAbs, StrLenLat> {
    fn visit(
        &self,
        env: &Environment<StrValAbs>,
        cache: &mut HashMap<u32, Vec<Expr<StrValAbs, StrLenLat>>>,
    ) -> Vec<Expr<StrValAbs, StrLenLat>> {
        match self {
            Func::Append(arg1, arg2) => arg1
                .visit(env, cache)
                .into_iter()
                .cartesian_product(arg2.visit(env, cache).into_iter())
                .map(|(a1, a2)| Expr::Call(Box::new(Func::Append(a1, a2))))
                .collect(),
            Func::Replace(arg1, arg2, arg3) => arg1
                .visit(env, cache)
                .into_iter()
                .cartesian_product(arg2.visit(env, cache).into_iter())
                .cartesian_product(arg3.visit(env, cache).into_iter())
                .map(|((a1, a2), a3)| Expr::Call(Box::new(Func::Replace(a1, a2, a3))))
                .collect(),
            Func::Substr(arg1, arg2, arg3) => arg1
                .visit(env, cache)
                .into_iter()
                .cartesian_product(arg2.visit(env, cache).into_iter())
                .cartesian_product(arg3.visit(env, cache).into_iter())
                .map(|((a1, a2), a3)| Expr::Call(Box::new(Func::Substr(a1, a2, a3))))
                .collect(),
            Func::Add(arg1, arg2) => arg1
                .visit(env, cache)
                .into_iter()
                .cartesian_product(arg2.visit(env, cache).into_iter())
                .map(|(a1, a2)| Expr::Call(Box::new(Func::Add(a1, a2))))
                .collect(),
            Func::Sub(arg1, arg2) => arg1
                .visit(env, cache)
                .into_iter()
                .cartesian_product(arg2.visit(env, cache).into_iter())
                .map(|(a1, a2)| Expr::Call(Box::new(Func::Sub(a1, a2))))
                .collect(),
            Func::Len(arg) => arg
                .visit(env, cache)
                .into_iter()
                .map(|a| Expr::Call(Box::new(Func::Len(a))))
                .collect(),
            Func::At(arg1, arg2) => arg1
                .visit(env, cache)
                .into_iter()
                .cartesian_product(arg2.visit(env, cache).into_iter())
                .map(|(a1, a2)| Expr::Call(Box::new(Func::At(a1, a2))))
                .collect(),
            Func::ToStr(arg) => arg
                .visit(env, cache)
                .into_iter()
                .map(|a| Expr::Call(Box::new(Func::ToStr(a))))
                .collect(),
            Func::ToInt(arg) => arg
                .visit(env, cache)
                .into_iter()
                .map(|a| Expr::Call(Box::new(Func::ToInt(a))))
                .collect(),
            Func::IndexOf(arg1, arg2, arg3) => arg1
                .visit(env, cache)
                .into_iter()
                .cartesian_product(arg2.visit(env, cache).into_iter())
                .cartesian_product(arg3.visit(env, cache).into_iter())
                .map(|((a1, a2), a3)| Expr::Call(Box::new(Func::IndexOf(a1, a2, a3))))
                .collect(),
            Func::PrefixOf(arg1, arg2) => arg1
                .visit(env, cache)
                .into_iter()
                .cartesian_product(arg2.visit(env, cache).into_iter())
                .map(|(a1, a2)| Expr::Call(Box::new(Func::PrefixOf(a1, a2))))
                .collect(),
            Func::SuffixOf(arg1, arg2) => arg1
                .visit(env, cache)
                .into_iter()
                .cartesian_product(arg2.visit(env, cache).into_iter())
                .map(|(a1, a2)| Expr::Call(Box::new(Func::SuffixOf(a1, a2))))
                .collect(),
            Func::Contains(arg1, arg2) => arg1
                .visit(env, cache)
                .into_iter()
                .cartesian_product(arg2.visit(env, cache).into_iter())
                .map(|(a1, a2)| Expr::Call(Box::new(Func::Contains(a1, a2))))
                .collect(),
        }
    }
}
