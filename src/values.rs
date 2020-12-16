pub trait Lattice: PartialOrd + PartialEq {
    // fn meet(&self, other: &Self) -> Self;
    // fn join(&self, other: &Self) -> Self;
    fn top() -> Self;
    fn bot() -> Self;
}

pub trait Value: Clone {
    fn is_abstract(&self) -> bool;

    fn is_concrete(&self) -> bool {
        !self.is_abstract()
    }
}
