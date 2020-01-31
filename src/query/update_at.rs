use crate::core::{ Named, QueryFail };
use crate::query::{ Query, Length };
use crate::data::Data;
use rand::Rng;

pub trait UpdateAt: Length {
    type Type: Data;
    fn update_at(&mut self, i: usize, val: &Self::Type);
}

pub struct UpdateAtQuery<T, C>(std::marker::PhantomData<(T, C)>);

impl<T, C> Named for UpdateAtQuery<T, C> {
    fn name() -> String { format!("UpdateAt") }
}

impl<T, C> Query for UpdateAtQuery<T, C> where
    T: UpdateAt,
    C: UpdateAt<Type=T::Type> {
    
        type Target = T;
        type Checker = C;
        fn verify<R: Rng>(gen: &mut R, target: &mut T, checker: &mut C) -> Result<(), QueryFail> {
            let i = gen.gen_range(0, target.length());
            let value = T::Type::generate(gen);
            target.update_at(i, &value);
            checker.update_at(i, &value);
            Ok(())
        }
}
