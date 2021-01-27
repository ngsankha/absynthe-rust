use absynthe::concrete::StrVal;
use absynthe::environment::Environment;
use absynthe::interpreter::EvalResult;
use absynthe::linear::LinearExpr;
use absynthe::types::{TypeLattice, TypeValues};
// use absynthe::strlenlat::StrLenLat;
use absynthe::synthesizer::{Context, Synthesizer};

#[test]
fn bikes() {
    let consts = vec![
        StrVal::from(0),
        StrVal::from(1),
        StrVal::from(2),
        StrVal::from(3),
        StrVal::from(4),
        StrVal::from(5),
        StrVal::from(" ".to_string()),
    ];

    let mut env = Environment::new();
    env.put("arg0".to_string(), TypeValues::Abs(TypeLattice::String));

    // let target = LinearExpr::from("x".to_string()) - LinearExpr::from(3);

    let mut ctx = Context::new(&consts, &env);

    let test = |f: Box<dyn Fn(&[StrVal]) -> EvalResult<StrVal>>| {
        f(&[StrVal::from("Ducati100".to_string())]) == Ok(StrVal::from("Ducati".to_string()))
            && f(&[StrVal::from("Honda125".to_string())]) == Ok(StrVal::from("Honda".to_string()))
            && f(&[StrVal::from("Ducati250".to_string())]) == Ok(StrVal::from("Ducati".to_string()))
            && f(&[StrVal::from("Honda250".to_string())]) == Ok(StrVal::from("Honda".to_string()))
            && f(&[StrVal::from("Honda550".to_string())]) == Ok(StrVal::from("Honda".to_string()))
            && f(&[StrVal::from("Ducati125".to_string())]) == Ok(StrVal::from("Ducati".to_string()))
    };

    let prog = &Synthesizer::synthesize(&mut ctx, TypeLattice::String, &env, Box::new(test))[0];

    assert_eq!("(substr arg0 0 (- (len arg0) 3))", format!("{}", prog));
}
