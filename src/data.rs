pub mod modint;

use crate::core::Named;
use rand::Rng;

pub struct M998244353();
impl modint::Mod for M998244353 {
    fn m() -> u64 { 998244353 }
}

pub type Mod998244353 = modint::ModInt<M998244353>;

pub trait Data: Clone + std::fmt::Debug + std::cmp::Eq + Named {
    fn generate<R: Rng>(gen: &mut R) -> Self;
}
