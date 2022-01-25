use crate::concrete::PyVal;
use crate::environment::Environment;
use crate::interpreter::EvalResult;
use crate::interpreter::Evaluable;
use crate::interpreter::SynthesisVisitor;
use crate::pythonlang::{Expr, Func, Term};
use crate::values::Lattice;
use crate::values::MixedValue;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TypeLattice {
    Top,
    String,
    Integer,
    Bool,
    Bot,
}

impl Lattice for TypeLattice {
    fn top() -> Self {
        Self::Top
    }

    fn bot() -> Self {
        Self::Bot
    }
}

impl Display for TypeLattice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Top => write!(f, "⊤"),
            Self::String => write!(f, "Str"),
            Self::Integer => write!(f, "Int"),
            Self::Bool => write!(f, "Bool"),
            Self::Bot => write!(f, "⊥"),
        }
    }
}

impl PartialOrd for TypeLattice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else {
            match (self, other) {
                (Self::Top, _) => Some(Ordering::Greater),
                (Self::Bot, _) => Some(Ordering::Less),
                (_, Self::Top) => Some(Ordering::Less),
                (_, Self::Bot) => Some(Ordering::Greater),
                _ => None,
            }
        }
    }
}

pub type TypeValues = MixedValue<PyVal, TypeLattice>;

impl TryFrom<TypeValues> for TypeLattice {
    type Error = &'static str;

    fn try_from(value: TypeValues) -> Result<Self, Self::Error> {
        match value {
            TypeValues::Abs(t) => Ok(t),
            TypeValues::Conc(v) => match v {
                PyVal::Int(_) => Ok(TypeLattice::Integer),
            },
        }
    }
}

impl Evaluable<TypeValues> for Expr<TypeValues, TypeLattice> {
    fn eval(&self, env: &Environment<TypeValues>) -> EvalResult<TypeValues> {
        match self {
            Self::Const(v) => Ok(v.clone()),
            Self::Var(x) => env
                .get(x.clone())
                .map(|v| v.clone())
                .ok_or_else(|| "variable not found"),
            Self::GetItem(recv, expr) => recv.get_item(expr),
            Self::Call(recv, call) => call.eval(recv, env),
            Self::Hole(abs, _) => Ok(TypeValues::from_abstract(abs.clone())),
            _ => unreachable!(),
        }
    }
}

// impl Evaluable<TypeValues> for Func<TypeValues, TypeLattice> {
//     fn eval(&self, env: &Environment<TypeValues>) -> EvalResult<TypeValues> {
//         match self {
//             Func::Append(arg1, arg2) => {
//                 let arg1_evaled = arg1.eval(env);
//                 let arg2_evaled = arg2.eval(env);
//                 match (arg1_evaled, arg2_evaled) {
//                     (
//                         Ok(TypeValues::Abs(TypeLattice::String)),
//                         Ok(TypeValues::Abs(TypeLattice::String)),
//                     ) => Ok(TypeValues::Abs(TypeLattice::String)),
//                     _ => Err("append: type error"),
//                 }
//             }
//             Func::Replace(arg1, arg2, arg3) => {
//                 let arg1_evaled = arg1.eval(env);
//                 let arg2_evaled = arg2.eval(env);
//                 let arg3_evaled = arg3.eval(env);
//                 match (arg1_evaled, arg2_evaled, arg3_evaled) {
//                     (
//                         Ok(TypeValues::Abs(TypeLattice::String)),
//                         Ok(TypeValues::Abs(TypeLattice::String)),
//                         Ok(TypeValues::Abs(TypeLattice::String)),
//                     ) => Ok(TypeValues::Abs(TypeLattice::String)),
//                     _ => Err("replace: type error"),
//                 }
//             }
//             Func::Substr(arg1, arg2, arg3) => {
//                 let arg1_evaled = arg1.eval(env);
//                 let arg2_evaled = arg2.eval(env);
//                 let arg3_evaled = arg3.eval(env);
//                 match (arg1_evaled, arg2_evaled, arg3_evaled) {
//                     (
//                         Ok(TypeValues::Abs(TypeLattice::String)),
//                         Ok(TypeValues::Abs(TypeLattice::Integer)),
//                         Ok(TypeValues::Abs(TypeLattice::Integer)),
//                     ) => Ok(TypeValues::Abs(TypeLattice::String)),
//                     _ => Err("substr: type error"),
//                 }
//             }
//             Func::Add(arg1, arg2) => {
//                 let arg1_evaled = arg1.eval(env);
//                 let arg2_evaled = arg2.eval(env);
//                 match (arg1_evaled, arg2_evaled) {
//                     (
//                         Ok(TypeValues::Abs(TypeLattice::Integer)),
//                         Ok(TypeValues::Abs(TypeLattice::Integer)),
//                     ) => Ok(TypeValues::Abs(TypeLattice::Integer)),
//                     _ => Err("add: type error"),
//                 }
//             }
//             Func::Sub(arg1, arg2) => {
//                 let arg1_evaled = arg1.eval(env);
//                 let arg2_evaled = arg2.eval(env);
//                 match (arg1_evaled, arg2_evaled) {
//                     (
//                         Ok(TypeValues::Abs(TypeLattice::Integer)),
//                         Ok(TypeValues::Abs(TypeLattice::Integer)),
//                     ) => Ok(TypeValues::Abs(TypeLattice::Integer)),
//                     _ => Err("sub: type error"),
//                 }
//             }
//             Func::Len(arg1) => {
//                 let arg1_evaled = arg1.eval(env);
//                 match arg1_evaled {
//                     Ok(TypeValues::Abs(TypeLattice::String)) => {
//                         Ok(TypeValues::Abs(TypeLattice::Integer))
//                     }
//                     _ => Err("len: type error"),
//                 }
//             }
//             Func::At(arg1, arg2) => {
//                 let arg1_evaled = arg1.eval(env);
//                 let arg2_evaled = arg2.eval(env);
//                 match (arg1_evaled, arg2_evaled) {
//                     (
//                         Ok(TypeValues::Abs(TypeLattice::String)),
//                         Ok(TypeValues::Abs(TypeLattice::Integer)),
//                     ) => Ok(TypeValues::Abs(TypeLattice::String)),
//                     _ => Err("at: type error"),
//                 }
//             }
//             Func::ToStr(arg1) => {
//                 let arg1_evaled = arg1.eval(env);
//                 match arg1_evaled {
//                     Ok(TypeValues::Abs(TypeLattice::Integer)) => {
//                         Ok(TypeValues::Abs(TypeLattice::String))
//                     }
//                     _ => Err("tostr: type error"),
//                 }
//             }
//             Func::ToInt(arg1) => {
//                 let arg1_evaled = arg1.eval(env);
//                 match arg1_evaled {
//                     Ok(TypeValues::Abs(TypeLattice::String)) => {
//                         Ok(TypeValues::Abs(TypeLattice::Integer))
//                     }
//                     _ => Err("toint: type error"),
//                 }
//             }
//             Func::IndexOf(arg1, arg2, arg3) => {
//                 let arg1_evaled = arg1.eval(env);
//                 let arg2_evaled = arg2.eval(env);
//                 let arg3_evaled = arg3.eval(env);
//                 match (arg1_evaled, arg2_evaled, arg3_evaled) {
//                     (
//                         Ok(TypeValues::Abs(TypeLattice::String)),
//                         Ok(TypeValues::Abs(TypeLattice::String)),
//                         Ok(TypeValues::Abs(TypeLattice::Integer)),
//                     ) => Ok(TypeValues::Abs(TypeLattice::Integer)),
//                     _ => Err("indexof: type error"),
//                 }
//             }
//             Func::PrefixOf(arg1, arg2) => {
//                 let arg1_evaled = arg1.eval(env);
//                 let arg2_evaled = arg2.eval(env);
//                 match (arg1_evaled, arg2_evaled) {
//                     (
//                         Ok(TypeValues::Abs(TypeLattice::String)),
//                         Ok(TypeValues::Abs(TypeLattice::String)),
//                     ) => Ok(TypeValues::Abs(TypeLattice::Bool)),
//                     _ => Err("prefixof: type error"),
//                 }
//             }
//             Func::SuffixOf(arg1, arg2) => {
//                 let arg1_evaled = arg1.eval(env);
//                 let arg2_evaled = arg2.eval(env);
//                 match (arg1_evaled, arg2_evaled) {
//                     (
//                         Ok(TypeValues::Abs(TypeLattice::String)),
//                         Ok(TypeValues::Abs(TypeLattice::String)),
//                     ) => Ok(TypeValues::Abs(TypeLattice::Bool)),
//                     _ => Err("suffixof: type error"),
//                 }
//             }
//             Func::Contains(arg1, arg2) => {
//                 let arg1_evaled = arg1.eval(env);
//                 let arg2_evaled = arg2.eval(env);
//                 match (arg1_evaled, arg2_evaled) {
//                     (
//                         Ok(TypeValues::Abs(TypeLattice::String)),
//                         Ok(TypeValues::Abs(TypeLattice::String)),
//                     ) => Ok(TypeValues::Abs(TypeLattice::Bool)),
//                     _ => Err("contains: type error"),
//                 }
//             }
//         }
//     }
// }

// impl Evaluable<TypeValues> for Term<TypeValues, TypeLattice> {
//     fn eval(&self, env: &Environment<TypeValues>) -> EvalResult<TypeValues> {
//         let inner: &Expr<TypeValues, TypeLattice> = self;
//         inner.eval(env)
//     }
// }

// impl SynthesisVisitor<TypeValues, TypeLattice> for Term<TypeValues, TypeLattice> {
//     fn visit(
//         &self,
//         env: &Environment<TypeValues>,
//         cache: &mut HashMap<u32, Vec<Term<TypeValues, TypeLattice>>>,
//     ) -> Vec<Term<TypeValues, TypeLattice>> {
//         let inner: &Expr<TypeValues, TypeLattice> = self;
//         match inner {
//             Expr::If(cond, then, otherwise) => unreachable!(),
//             Expr::Call(f) => f.visit(env, cache),
//             Expr::Hole(abs, expr) => Expr::visit_hole(abs, expr, env, cache),
//             Expr::ConcHole(_) => unreachable!(),
//             Expr::DepHole => unreachable!(),
//             Expr::Const(c) => vec![self.clone()],
//             Expr::Var(v) => vec![self.clone()],
//         }
//     }
// }

// impl SynthesisVisitor<TypeValues, TypeLattice> for Func<TypeValues, TypeLattice> {
//     fn visit(
//         &self,
//         env: &Environment<TypeValues>,
//         cache: &mut HashMap<u32, Vec<Term<TypeValues, TypeLattice>>>,
//     ) -> Vec<Term<TypeValues, TypeLattice>> {
//         match self {
//             Func::Append(arg1, arg2) => arg1
//                 .visit(env, cache)
//                 .into_iter()
//                 .cartesian_product(arg2.visit(env, cache).into_iter())
//                 .map(|(a1, a2)| Rc::new(Expr::Call(Func::Append(a1, a2))))
//                 .collect(),
//             Func::Replace(arg1, arg2, arg3) => arg1
//                 .visit(env, cache)
//                 .into_iter()
//                 .cartesian_product(arg2.visit(env, cache).into_iter())
//                 .cartesian_product(arg3.visit(env, cache).into_iter())
//                 .map(|((a1, a2), a3)| Rc::new(Expr::Call(Func::Replace(a1, a2, a3))))
//                 .collect(),
//             Func::Substr(arg1, arg2, arg3) => arg1
//                 .visit(env, cache)
//                 .into_iter()
//                 .cartesian_product(arg2.visit(env, cache).into_iter())
//                 .cartesian_product(arg3.visit(env, cache).into_iter())
//                 .map(|((a1, a2), a3)| Rc::new(Expr::Call(Func::Substr(a1, a2, a3))))
//                 .collect(),
//             Func::Add(arg1, arg2) => arg1
//                 .visit(env, cache)
//                 .into_iter()
//                 .cartesian_product(arg2.visit(env, cache).into_iter())
//                 .map(|(a1, a2)| Rc::new(Expr::Call(Func::Add(a1, a2))))
//                 .collect(),
//             Func::Sub(arg1, arg2) => arg1
//                 .visit(env, cache)
//                 .into_iter()
//                 .cartesian_product(arg2.visit(env, cache).into_iter())
//                 .map(|(a1, a2)| Rc::new(Expr::Call(Func::Sub(a1, a2))))
//                 .collect(),
//             Func::Len(arg) => arg
//                 .visit(env, cache)
//                 .into_iter()
//                 .map(|a| Rc::new(Expr::Call(Func::Len(a))))
//                 .collect(),
//             Func::At(arg1, arg2) => arg1
//                 .visit(env, cache)
//                 .into_iter()
//                 .cartesian_product(arg2.visit(env, cache).into_iter())
//                 .map(|(a1, a2)| Rc::new(Expr::Call(Func::At(a1, a2))))
//                 .collect(),
//             Func::ToStr(arg) => arg
//                 .visit(env, cache)
//                 .into_iter()
//                 .map(|a| Rc::new(Expr::Call(Func::ToStr(a))))
//                 .collect(),
//             Func::ToInt(arg) => arg
//                 .visit(env, cache)
//                 .into_iter()
//                 .map(|a| Rc::new(Expr::Call(Func::ToInt(a))))
//                 .collect(),
//             Func::IndexOf(arg1, arg2, arg3) => arg1
//                 .visit(env, cache)
//                 .into_iter()
//                 .cartesian_product(arg2.visit(env, cache).into_iter())
//                 .cartesian_product(arg3.visit(env, cache).into_iter())
//                 .map(|((a1, a2), a3)| Rc::new(Expr::Call(Func::IndexOf(a1, a2, a3))))
//                 .collect(),
//             Func::PrefixOf(arg1, arg2) => arg1
//                 .visit(env, cache)
//                 .into_iter()
//                 .cartesian_product(arg2.visit(env, cache).into_iter())
//                 .map(|(a1, a2)| Rc::new(Expr::Call(Func::PrefixOf(a1, a2))))
//                 .collect(),
//             Func::SuffixOf(arg1, arg2) => arg1
//                 .visit(env, cache)
//                 .into_iter()
//                 .cartesian_product(arg2.visit(env, cache).into_iter())
//                 .map(|(a1, a2)| Rc::new(Expr::Call(Func::SuffixOf(a1, a2))))
//                 .collect(),
//             Func::Contains(arg1, arg2) => arg1
//                 .visit(env, cache)
//                 .into_iter()
//                 .cartesian_product(arg2.visit(env, cache).into_iter())
//                 .map(|(a1, a2)| Rc::new(Expr::Call(Func::Contains(a1, a2))))
//                 .collect(),
//         }
//     }
// }

// impl Expr<TypeValues, TypeLattice> {
//     fn visit_hole(
//         target: &TypeLattice,
//         expr: &Option<Func<TypeValues, TypeLattice>>,
//         env: &Environment<TypeValues>,
//         cache: &mut HashMap<u32, Vec<Term<TypeValues, TypeLattice>>>,
//     ) -> Vec<Term<TypeValues, TypeLattice>> {
//         // assert_eq!(expr, None as &Option<Func<TypeValues, TypeLattice>>);
//         let strhole = Rc::new(Expr::Hole(TypeLattice::String, None));
//         let inthole = Rc::new(Expr::Hole(TypeLattice::Integer, None));
//         match target {
//             TypeLattice::Bot => vec![],
//             TypeLattice::String => vec![
//                 Rc::new(Expr::Call(Func::Append(strhole.clone(), strhole.clone()))),
//                 Rc::new(Expr::Call(Func::Replace(
//                     strhole.clone(),
//                     strhole.clone(),
//                     strhole.clone(),
//                 ))),
//                 Rc::new(Expr::Call(Func::Substr(
//                     strhole.clone(),
//                     inthole.clone(),
//                     inthole.clone(),
//                 ))),
//                 Rc::new(Expr::Call(Func::At(strhole.clone(), inthole.clone()))),
//                 Rc::new(Expr::Call(Func::ToStr(inthole.clone()))),
//             ]
//             .into_iter()
//             .chain(cache[&0].clone().into_iter().filter(|e| {
//                 e.eval(env)
//                     .map(|v| TypeLattice::try_from(v).unwrap())
//                     .map_or_else(|_| false, |v| v <= TypeLattice::String)
//             }))
//             .collect(),
//             TypeLattice::Integer => vec![
//                 Rc::new(Expr::Call(Func::Add(inthole.clone(), inthole.clone()))),
//                 Rc::new(Expr::Call(Func::Sub(inthole.clone(), inthole.clone()))),
//                 Rc::new(Expr::Call(Func::Len(strhole.clone()))),
//                 Rc::new(Expr::Call(Func::ToInt(strhole.clone()))),
//                 Rc::new(Expr::Call(Func::IndexOf(
//                     strhole.clone(),
//                     strhole.clone(),
//                     inthole.clone(),
//                 ))),
//             ]
//             .into_iter()
//             .chain(cache[&0].clone().into_iter().filter(|e| {
//                 e.eval(env)
//                     .map(|v| TypeLattice::try_from(v).unwrap())
//                     .map_or_else(|_| false, |v| v <= TypeLattice::Integer)
//             }))
//             .collect(),
//             TypeLattice::Bool => vec![
//                 Rc::new(Expr::Call(Func::PrefixOf(strhole.clone(), strhole.clone()))),
//                 Rc::new(Expr::Call(Func::SuffixOf(strhole.clone(), strhole.clone()))),
//                 Rc::new(Expr::Call(Func::Contains(strhole.clone(), strhole.clone()))),
//             ]
//             .into_iter()
//             .chain(cache[&0].clone().into_iter().filter(|e| {
//                 e.eval(env)
//                     .map(|v| TypeLattice::try_from(v).unwrap())
//                     .map_or_else(|_| false, |v| v <= TypeLattice::Bool)
//             }))
//             .collect(),
//             TypeLattice::Top => Self::visit_hole(&TypeLattice::String, expr, env, cache)
//                 .into_iter()
//                 .chain(Self::visit_hole(&TypeLattice::Integer, expr, env, cache).into_iter())
//                 .chain(Self::visit_hole(&TypeLattice::Bool, expr, env, cache).into_iter())
//                 .collect(),
//         }
//     }
// }

// impl From<Term<TypeValues, TypeLattice>> for Expr<StrVal, TypeLattice> {
//     fn from(value: Term<TypeValues, TypeLattice>) -> Self {
//         let inner: &Expr<TypeValues, TypeLattice> = &value;
//         match inner {
//             Expr::Const(TypeValues::Conc(v)) => Expr::Const(v.clone()),
//             Expr::Const(_) => unreachable!(),
//             Expr::Var(x) => Expr::Var(x.clone()),
//             Expr::Call(f) => Expr::Call(Func::from(f.clone())),
//             Expr::If(_, _, _) => unimplemented!(),
//             _ => unreachable!(),
//         }
//     }
// }

// impl From<Func<TypeValues, TypeLattice>> for Func<StrVal, TypeLattice> {
//     fn from(value: Func<TypeValues, TypeLattice>) -> Self {
//         match value {
//             Func::Append(arg1, arg2) => {
//                 Func::Append(Rc::new(Expr::from(arg1)), Rc::new(Expr::from(arg2)))
//             }
//             Func::Replace(arg1, arg2, arg3) => Func::Replace(
//                 Rc::new(Expr::from(arg1)),
//                 Rc::new(Expr::from(arg2)),
//                 Rc::new(Expr::from(arg3)),
//             ),
//             Func::Substr(arg1, arg2, arg3) => Func::Substr(
//                 Rc::new(Expr::from(arg1)),
//                 Rc::new(Expr::from(arg2)),
//                 Rc::new(Expr::from(arg3)),
//             ),
//             Func::Add(arg1, arg2) => {
//                 Func::Add(Rc::new(Expr::from(arg1)), Rc::new(Expr::from(arg2)))
//             }
//             Func::Sub(arg1, arg2) => {
//                 Func::Sub(Rc::new(Expr::from(arg1)), Rc::new(Expr::from(arg2)))
//             }
//             Func::Len(arg1) => Func::Len(Rc::new(Expr::from(arg1))),
//             Func::At(arg1, arg2) => Func::At(Rc::new(Expr::from(arg1)), Rc::new(Expr::from(arg2))),
//             Func::ToStr(arg1) => Func::ToStr(Rc::new(Expr::from(arg1))),
//             Func::ToInt(arg1) => Func::ToInt(Rc::new(Expr::from(arg1))),
//             Func::IndexOf(arg1, arg2, arg3) => Func::IndexOf(
//                 Rc::new(Expr::from(arg1)),
//                 Rc::new(Expr::from(arg2)),
//                 Rc::new(Expr::from(arg3)),
//             ),
//             Func::PrefixOf(arg1, arg2) => {
//                 Func::PrefixOf(Rc::new(Expr::from(arg1)), Rc::new(Expr::from(arg2)))
//             }
//             Func::SuffixOf(arg1, arg2) => {
//                 Func::SuffixOf(Rc::new(Expr::from(arg1)), Rc::new(Expr::from(arg2)))
//             }
//             Func::Contains(arg1, arg2) => {
//                 Func::Contains(Rc::new(Expr::from(arg1)), Rc::new(Expr::from(arg2)))
//             }
//         }
//     }
// }
