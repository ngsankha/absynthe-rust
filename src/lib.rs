pub mod r#abstract;
pub mod concrete;
pub mod interpreter;

// pub fn interp<T: Value, Clone>(expr: &Expr<T>, env: &HashMap<String, T>) -> T {
//     match expr {
//         Expr::Const(v) => *v.clone(),
//         Expr::Var(x) => match env.get(x) {
//             Some(val) => *val.clone(),
//             None => T::error()
//         }
//         Expr::Call(call) => eval_call(call, env)
//     }
// }

// fn eval_call<T>(call: &Func<T>, env: &HashMap<String, T>) -> T {
//     match call {
//         Func::Append(s1, s2) => str_append(interp(s1, env), interp(s2, env)),
//         // Func::Replace(s1, s2, s3) => str_replace(interp(s1, env), interp(s2, env), interp(s3, env)),
//         // Func::Substr(s1, s2, s3) => str_substr(interp(s1, env), interp(s2, env), interp(s3, env)),
//         // Func::Add(i, j) => int_add(interp(i, env), interp(j, env)),
//         // Func::Sub(i, j) => int_sub(interp(i, env), interp(j, env)),
//         // Func::Len(s) => str_len(interp(s, env))
//     }
// }

// fn str_append<T>(v1: StrVal, v2: StrVal) -> StrVal {
//     match (v1, v2) {
//         (StrVal::Str(s1), StrVal::Str(s2)) => StrVal::Str(s1 + &s2),
//         _ => StrVal::error()
//     }
// }

// fn abs_str_append(v1: StringLenLattice, v2: StringLenLattice) -> StringLenLattice {
//     match (v1, v2) {
//         (StringLenLattice::Top, _) => StringLenLattice::Top,
//         (_, StringLenLattice::Top) => StringLenLattice::Top,
//         (StringLenLattice::Len(l1), StringLenLattice::Len(l2)) => StringLenLattice::Len(l1 + l2),
//         (StringLenLattice::Len(_), StringLenLattice::Bot) => StringLenLattice::Bot,
//         (StringLenLattice::Bot, StringLenLattice::Len(_)) => StringLenLattice::Bot,
//         (StringLenLattice::Bot, StringLenLattice::Bot) => StringLenLattice::Bot
//     }
// }

