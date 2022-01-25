use crate::environment::Environment;
use crate::interpreter::{EvalResult, Evaluable};
use crate::linear::LinearExpr;
use crate::pythonlang::{Expr, Func};
use crate::types::TypeLattice;
use crate::values::{Lattice, Value};
use pyo3::types::IntoPyDict;
use pyo3::Python;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PyVal {
    Int(LinearExpr),
}

impl Value for PyVal {
    fn is_abstract(&self) -> bool {
        false
    }
}

impl Display for PyVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PyVal::Int(i) => write!(f, "{}", i),
        }
    }
}

impl From<i32> for PyVal {
    fn from(item: i32) -> Self {
        PyVal::Int(LinearExpr::from(item))
    }
}

impl From<LinearExpr> for PyVal {
    fn from(item: LinearExpr) -> Self {
        PyVal::Int(item)
    }
}

impl Evaluable<PyVal> for Expr<PyVal, TypeLattice> {
    fn eval(&self, env: &Environment<PyVal>) -> EvalResult<PyVal> {
        Python::with_gil(|py| {
            let result = py.eval("[i * 10 for i in range(5)]", None, None).unwrap();
            let res: Vec<i64> = result.extract().unwrap();
            assert_eq!(res, vec![0, 10, 20, 30, 40])
        });
        Ok(PyVal::Int(LinearExpr::from(3)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::types::PyDict;
    use pyo3::types::PyModule;

    #[test]
    fn python_interp() {
        Python::with_gil(|py| {
            // let locals = [("pd", py.import("pandas").unwrap())].into_py_dict(py);
            let snippet = PyModule::from_code(
                py,
                r#"
def syn():
    return [i * 10 for i in range(5)]

def test():
    return syn() == [0, 10, 20, 30, 40]
            "#,
                "gen.py",
                "gen",
            )
            .unwrap();
            let res: bool = snippet.call0("test").unwrap().extract().unwrap();
            assert!(res)
        });
    }
}
