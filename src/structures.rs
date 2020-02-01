pub mod vec;
pub mod wrong3;

use crate::core::Named;

pub trait Structure: Named {
    fn new() -> Self;
}
