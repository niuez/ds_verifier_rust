use crate::core::{ Named, QueryFail };
use crate::query::{ Query, Length, LengthQuery };
use crate::data::Data;
use crate::debug::DebuggableRng;

pub trait AccessAt: Length {
    type Type: Data;
    fn access_at(&self, i: usize) -> Self::Type;
}

pub struct AccessAtQuery<T, C>(std::marker::PhantomData<(T, C)>);

impl<T, C> Named for AccessAtQuery<T, C> {
    fn name() -> String { format!("AccessAt") }
}

impl<T, C> Query for AccessAtQuery<T, C> where
    T: AccessAt,
    C: AccessAt<Type=T::Type> {
        type Target = T;
        type Checker = C;
    fn verify<R: DebuggableRng<T, C>>(gen: &mut R, target: &mut T, checker: &mut C) -> Result<(), QueryFail> {
        LengthQuery::<T, C>::verify(gen, target, checker)?;

        let i = gen.gen_range(0, target.length());
        gen.debugtrace(target, checker);

        let t_res = target.access_at(i);
        let c_res = checker.access_at(i);
        if t_res == c_res {
            Ok(())
        }
        else {
            Err( QueryFail {
                fail_query: Self::name(),
                fail_detail: format!("target results {:?} but checker results {:?}", t_res, c_res),
            })
        }
    }
}
