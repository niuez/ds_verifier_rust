use crate::core::Named;
use crate::query::{ Length, AccessAt, UpdateAt };
use crate::data::Data;
use crate::structures::Structure;

impl<T: Named> Structure for Vec<T> {
    fn new() -> Self { Vec::new() }
}

impl<T: Named> Named for Vec<T> {
    fn name() -> String { format!("std::vec::Vec<{}>", T::name()) }
}

impl<T> Length for Vec<T> {
    fn length(&self) -> usize { self.len() }
}

impl<T: Data> AccessAt for Vec<T> {
    type Type = T;
    fn access_at(&self, i: usize) -> T { self[i].clone() }
}

impl<T: Data> UpdateAt for Vec<T> {
    type Type = T;
    fn update_at(&mut self, i: usize, val: &T) { self[i] = val.clone(); }
}
