use crate::core::{ Named, QueryFail };
use crate::query::Query;
use rand::Rng;

pub trait Length {
    fn length(&self) -> usize;
}

pub struct LengthQuery<T, C>(std::marker::PhantomData<(T, C)>);

impl<T, C> Named for LengthQuery<T, C> {
    fn name() -> String { format!("Length") }
}

impl<T: Length, C: Length> Query for LengthQuery<T, C> {
    type Target = T;
    type Checker = C;
    fn verify<R: Rng>(_: &mut R, target: &mut T, checker: &mut C) -> Result<(), QueryFail> {
        let t_len = target.length();
        let c_len = checker.length();
        if t_len == c_len {
            Ok(())
        }
        else {
            Err( QueryFail {
                fail_query: Self::name(),
                fail_detail: format!("target's length {:?} but checker results {:?}", t_len, c_len),
            })
        }
    }
}
