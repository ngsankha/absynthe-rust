use crate::environment::Environment;
use crate::pythonlang::Term;
use crate::values::{Lattice, Value};
use std::collections::HashMap;
use std::fmt::Debug;

pub type EvalResult<T: Value> = Result<T, &'static str>;

pub trait Evaluable<T: Value + Debug> {
    fn eval(&self, env: &Environment<T>) -> EvalResult<T>;
}

pub trait ConcretizedSynth<T: Value, U: Lattice> {
    fn concretize(
        env: &Environment<T>,
        size: u32,
        cache: &mut HashMap<u32, Vec<Term<T, U>>>,
    ) -> Vec<Term<T, U>>;
}

pub trait SynthesisVisitor<T: Value, U: Lattice> {
    fn visit(
        &self,
        env: &Environment<T>,
        cache: &mut HashMap<u32, Vec<Term<T, U>>>,
    ) -> Vec<Term<T, U>>;
}
