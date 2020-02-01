use crate::core::{ Named, QueryFail };
use crate::query::Query;
use crate::debug::DebuggableRng;

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
    fn verify<R: DebuggableRng<T, C>>(gen: &mut R, target: &mut T, checker: &mut C) -> Result<(), QueryFail> {
        let t_len = target.length();
        let c_len = checker.length();
        if t_len == c_len {
            Ok(())
        }
        else {
            Err( QueryFail::new(Self::name(), format!("target's length {:?} but checker results {:?}", t_len, c_len), gen))
        }
    }
}
