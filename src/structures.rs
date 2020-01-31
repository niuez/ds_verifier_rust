pub mod vec;

use crate::core::Named;

pub trait Structure: Named {
    fn new() -> Self;
}
