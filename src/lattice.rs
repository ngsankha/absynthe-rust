use std::cmp::Ordering;

trait Lattice {
  fn join(&self, other: &Self) -> Self
}

#[derive(Debug)]
enum StrLenLattice {
  Top,
  Elem(usize),
  Bot
}

impl Lattice for StrLenLattice {
  fn join(&self, other: &Self) -> Self {
    match self {
      StrLenLattice::Top => self,
      StrLenLattice::Elem(v1) => match other {
        StrLenLattice::Top => StrLenLattice::Top,
        StrLenLattice::Elem(v2) => if v1 > v2 {
          StrLenLattice::Elem(v1)
        } else {
          StrLenLattice::Elem(v2)
        },
        StrLenLattice::Bot => self
      }
      StrLenLattice::Bot => other
    }
  }
}

impl PartialOrd for StrLenLattice {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match self {
      StrLenLattice::Top => match other {
        StrLenLattice::Top => Some(Ordering::Equal),
        _ => Some(Ordering::Greater)
      },
      StrLenLattice::Elem(x) => match other {
        StrLenLattice::Top => Some(Ordering::Less),
        StrLenLattice::Elem(y) => x.partial_cmp(y),
        StrLenLattice::Bot => Some(Ordering::Greater)
      },
      StrLenLattice::Bot => match other {
        StrLenLattice::Bot => Some(Ordering::Equal),
        _ => Some(Ordering::Less)
      }
    }
  }
}

impl PartialEq for StrLenLattice {
  fn eq(&self, other: &Self) -> bool {
    match self {
      StrLenLattice::Top => match other {
        StrLenLattice::Top => true,
        _ => false
      },
      StrLenLattice::Elem(x) => match other {
        StrLenLattice::Top => false,
        StrLenLattice::Elem(y) => x == y,
        StrLenLattice::Bot => false
      },
      StrLenLattice::Bot => match other {
        StrLenLattice::Bot => true,
        _ => false
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn lattice_eq() {
    assert_eq!(StrLenLattice::Top == StrLenLattice::Top, true);
    assert_eq!(StrLenLattice::Bot == StrLenLattice::Bot, true);
    assert_eq!(StrLenLattice::Elem(5) == StrLenLattice::Elem(5), true);
    assert_eq!(StrLenLattice::Top == StrLenLattice::Bot, false);
    assert_eq!(StrLenLattice::Top == StrLenLattice::Elem(5), false);
    assert_eq!(StrLenLattice::Bot == StrLenLattice::Elem(5), false);
  }

  #[test]
  fn lattice_order() {
    assert!(StrLenLattice::Bot <= StrLenLattice::Top);
    assert!(StrLenLattice::Bot <= StrLenLattice::Elem(5));
    assert!(StrLenLattice::Elem(5) <= StrLenLattice::Top);
    assert!(StrLenLattice::Elem(2) <= StrLenLattice::Elem(5));
  }

  #[test]
  fn interp_fail() {
    assert!(interp(Expr::Substr(Box::new(Expr::Len(Box::new(
      Expr::Val(Value::Str("hello world".to_string()))))), Box::new(Expr::Len(Box::new(
      Expr::Val(Value::Str("hello world".to_string()))))), Box::new(Expr::Len(Box::new(
      Expr::Val(Value::Str("hello world".to_string()))))))) == Value::Error)
  }

  #[test]
  fn interp_pass() {
    assert!(interp(Expr::Len(Box::new(
      Expr::Val(Value::Str("hello world".to_string()))))) == Value::Num(11))
  }
}

#[derive(Debug)]
enum Value {
  Error,
  Str(String),
  Num(usize)
}

impl PartialEq for Value {
  fn eq(&self, other: &Self) -> bool {
    match self {
      Value::Error => match other {
        Value::Error => true,
        _ => false
      },
      Value::Str(s1) => match other {
        Value::Str(s2) => s1 == s2,
        _ => false
      },
      Value::Num(n) => match other {
        Value::Num(m) => n == m,
        _ => false
      }
    }
  }
}

enum Expr {
  Val(Value),
  Append(Box<Expr>, Box<Expr>),
  Replace(Box<Expr>, Box<Expr>, Box<Expr>),
  Substr(Box<Expr>, Box<Expr>, Box<Expr>),
  Add(Box<Expr>, Box<Expr>),
  Sub(Box<Expr>, Box<Expr>),
  Len(Box<Expr>)
}

fn interp(e: Expr) -> Value {
  match e {
    Expr::Val(v) => v,
    Expr::Append(x, y) => str_append(*x, *y),
    Expr::Replace(x, y, z) => str_replace(*x, *y, *z),
    Expr::Substr(x, y, z) => str_substr(*x, *y, *z),
    Expr::Add(x, y) => num_add(*x, *y),
    Expr::Sub(x, y) => num_sub(*x, *y),
    Expr::Len(x) => str_len(*x)
  }
}

fn num_add(x: Expr, y: Expr) -> Value {
  let xv = interp(x);
  let yv = interp(y);
  match xv {
    Value::Num(i) => match yv {
      Value::Num(j) => Value::Num(i + j),
      _ => Value::Error
    },
    _ => Value::Error
  }
}

fn num_sub(x: Expr, y: Expr) -> Value {
  let xv = interp(x);
  let yv = interp(y);
  match xv {
    Value::Num(i) => match yv {
      Value::Num(j) => Value::Num(i - j),
      _ => Value::Error
    },
    _ => Value::Error
  }
}

fn str_len(x: Expr) -> Value {
  let xv = interp(x);
  match xv {
    Value::Str(s1) => Value::Num(s1.chars().count()),
    _ => Value::Error
  }
}

fn str_append(x: Expr, y: Expr) -> Value {
  let xv = interp(x);
  let yv = interp(y);
  match xv {
    Value::Str(s1) => match yv {
      Value::Str(s2) => Value::Str(s1 + &s2),
      _ => Value::Error
    },
    _ => Value::Error
  }
}

fn str_replace(x: Expr, y: Expr, z: Expr) -> Value {
  let xv = interp(x);
  let yv = interp(y);
  let zv = interp(z);
  match xv {
    Value::Str(s1) => match yv {
      Value::Str(s2) => match zv {
        Value::Str(s3) => Value::Str(s1.replace(&s2, &s3)),
        _ => Value::Error
      },
      _ => Value::Error
    },
    _ => Value::Error
  }
}

fn str_substr(x: Expr, y: Expr, z: Expr) -> Value {
  let xv = interp(x);
  let yv = interp(y);
  let zv = interp(z);
  match xv {
    Value::Str(s1) => match yv {
      Value::Num(i) => match zv {
        Value::Num(j) => Value::Str(s1.chars().skip(i).take(j).collect()),
        _ => Value::Error
      },
      _ => Value::Error
    },
    _ => Value::Error
  }
}

fn abstraction(v: Value) -> StrLenLattice {
  match v {
    Value::Str(s) => StrLenLattice::Elem(s.chars().count()),
    _ => StrLenLattice::Bot
  }
}

fn abs_interp(e: Expr) -> StrLenLattice {
  match e {
    Expr::Val(v) => abstraction(v),
    Expr::Append(x, y) => abs_str_append(*x, *y),
    Expr::Replace(x, y, z) => abs_str_replace(*x, *y, *z),
    Expr::Substr(x, y, z) => abs_str_substr(*x, *y, *z),
    Expr::Add(x, y) => abs_num_add(*x, *y),
    Expr::Sub(x, y) => abs_num_sub(*x, *y),
    Expr::Len(x) => abs_str_len(*x)
  }
}

fn num_add(x: Expr, y: Expr) -> StrLenLattice {
  StrLenLattice::Bot
}

fn num_sub(x: Expr, y: Expr) -> StrLenLattice {
  StrLenLattice::Bot
}

fn str_len(x: Expr) -> StrLenLattice {
  abs_interp(x)
}

fn str_append(x: Expr, y: Expr) -> StrLenLattice {
  let xv = abs_interp(x);
  let yv = abs_interp(y);
  match xv {
    StrLenLattice::Elem(v1) => match yv {
      StrLenLattice::Elem(v2) => StrLenLattice::Elem(v1 + v2),
      _ => yv
    },
    _ => xv
  }
}

fn str_replace(x: Expr, y: Expr, z: Expr) -> StrLenLattice {
  StrLenLattice::Top
}

fn str_substr(x: Expr, y: Expr, z: Expr) -> Value {
  let xv = abs_interp(x);
  let yv = abs_interp(y);
  let zv = abs_interp(z);
  match xv {
    Value::Str(s1) => match yv {
      Value::Num(i) => match zv {
        Value::Num(j) => Value::Str(s1.chars().skip(i).take(j).collect()),
        _ => Value::Error
      },
      _ => Value::Error
    },
    _ => Value::Error
  }
}

fn main() {
  let prog = Expr::Substr(Box::new(Expr::Len(Box::new(
      Expr::Val(Value::Str("hello world".to_string()))))), Box::new(Expr::Len(Box::new(
      Expr::Val(Value::Str("hello world".to_string()))))), Box::new(Expr::Len(Box::new(
      Expr::Val(Value::Str("hello world".to_string()))))));
  // interp(prog);
  println!("{:?}", interp(prog));
  println!("Hello world!");
}

