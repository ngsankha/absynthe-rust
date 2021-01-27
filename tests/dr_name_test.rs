// use absynthe::concrete::StrVal;
// use absynthe::environment::Environment;
// use absynthe::interpreter::EvalResult;
// use absynthe::linear::LinearExpr;
// use absynthe::r#abstract::StrValAbs;
// use absynthe::strlenlat::StrLenLat;
// use absynthe::synthesizer::{Context, Synthesizer};

// #[test]
// #[ignore]
// fn dr_name() {
//     let consts = vec![
//         StrVal::from(0),
//         StrVal::from(1),
//         StrVal::from(2),
//         StrVal::from(" ".to_string()),
//         StrVal::from(".".to_string()),
//         StrVal::from("Dr.".to_string()),
//     ];

//     let mut env = Environment::new();
//     env.put(
//         "arg0".to_string(),
//         StrValAbs::Abs(StrLenLat::from("x".to_string())),
//     );

//     let target = LinearExpr::from("x".to_string()) - LinearExpr::from(3);

//     let mut ctx = Context::new(&consts, &env);

//     let test = |f: Box<dyn Fn(&[StrVal]) -> EvalResult<StrVal>>| {
//         f(&[StrVal::from("Nancy FreeHafer".to_string())])
//             == Ok(StrVal::from("Dr. Nancy".to_string()))
//             && f(&[StrVal::from("Andrew Cencici".to_string())])
//                 == Ok(StrVal::from("Dr. Andrew".to_string()))
//             && f(&[StrVal::from("Jan Kotas".to_string())])
//                 == Ok(StrVal::from("Dr. Jan".to_string()))
//             && f(&[StrVal::from("Mariya Sergienko".to_string())])
//                 == Ok(StrVal::from("Dr. Mariya".to_string()))
//     };

//     let prog = &Synthesizer::synthesize(&mut ctx, TypeLattice::String, &env, Box::new(test))[0];

//     assert_eq!(
//         "(append \"Dr.\" (append \" \" (substr arg0 0 (indexof arg0 \" \" 0))))",
//         format!("{}", prog)
//     );
// }
