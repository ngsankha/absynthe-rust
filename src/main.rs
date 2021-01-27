// use z3::ast::{Ast, Bool, Datatype, Dynamic, Int};
// use z3::{
//     Config, Context, DatatypeAccessor, DatatypeBuilder, DatatypeSort, SatResult, Solver, Sort,
// };

// fn addlat<'a>(lattice: &'a DatatypeSort, lhs: &'a Datatype, rhs: &'a Datatype) -> Dynamic<'a> {
//     let cond1 = Bool::or(
//         lhs.get_ctx(),
//         &[
//             &lattice.variants[0]
//                 .tester
//                 .apply(&[&lhs.clone().into()])
//                 .as_bool()
//                 .unwrap(),
//             &lattice.variants[0]
//                 .tester
//                 .apply(&[&rhs.clone().into()])
//                 .as_bool()
//                 .unwrap(),
//         ],
//     );

//     let cond2 = Bool::or(
//         lhs.get_ctx(),
//         &[
//             &lattice.variants[2]
//                 .tester
//                 .apply(&[&lhs.clone().into()])
//                 .as_bool()
//                 .unwrap(),
//             &lattice.variants[2]
//                 .tester
//                 .apply(&[&rhs.clone().into()])
//                 .as_bool()
//                 .unwrap(),
//         ],
//     );

//     cond1.ite(
//         &lattice.variants[0].constructor.apply(&[]),
//         &cond2.ite(
//             &lattice.variants[2].constructor.apply(&[]),
//             &lattice.variants[1].constructor.apply(&[&Int::add(
//                 lhs.get_ctx(),
//                 &[
//                     &lattice.variants[1].accessors[0]
//                         .apply(&[&lhs.clone().into()])
//                         .as_int()
//                         .unwrap(),
//                     &lattice.variants[1].accessors[0]
//                         .apply(&[&rhs.clone().into()])
//                         .as_int()
//                         .unwrap(),
//                 ],
//             )
//             .into()]),
//         ),
//     )
// }

use absynthe::concrete::StrVal;
use absynthe::environment::Environment;
use absynthe::interpreter::EvalResult;
use absynthe::synthesizer::Context;
use absynthe::synthesizer::Synthesizer;
use absynthe::types::{TypeLattice, TypeValues};

fn main() {
    // let cfg = Config::new();
    // let ctx = Context::new(&cfg);

    // let lattice = DatatypeBuilder::new(&ctx, "strlen")
    //     .variant("Top", vec![])
    //     .variant(
    //         "Len",
    //         vec![("Val", DatatypeAccessor::Sort(Sort::int(&ctx)))],
    //     )
    //     .variant("Bot", vec![])
    //     .finish();
    // let x = Datatype::new_const(&ctx, "x", &lattice.sort);
    // let len = lattice.variants[1]
    //     .constructor
    //     .apply(&[&Int::from_u64(&ctx, 2).into()])
    //     .as_datatype()
    //     .unwrap();
    // let s = Solver::new(&ctx);
    // s.assert(
    //     &addlat(&lattice, &x, &len)._eq(
    //         &lattice.variants[1]
    //             .constructor
    //             .apply(&[&Int::from_u64(&ctx, 6).into()]),
    //     ),
    // );

    // match s.check() {
    //     SatResult::Sat => {
    //         let m = s.get_model().unwrap();
    //         println!("{:?}", m.eval(&x));
    //     }
    //     _ => println!("ignore"),
    // }

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
    println!("{}", prog);
}
