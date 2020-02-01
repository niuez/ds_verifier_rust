use crate::query::{ Query, Length, LengthQuery };
use crate::core::{ Named, QueryFail, Number };
use crate::data::Data;
use crate::debug::DebuggableRng;

pub struct InitByVec<S, T, V, N>(std::marker::PhantomData<(S, T, V, N)>);

impl<S, T, V, N> Named for InitByVec<S, T, V, N> where
    V: Named, N: Number {
        fn name() -> String {
            format!("Init by Vec[{}; {}]", V::name(), N::N)
        }
    }

impl<S, T, V, N> Query for InitByVec<S, T, V, N> where
    S: From<Vec<V>> + Length,
    T: From<Vec<V>> + Length,
    V: Data,
    N: Number, {
        type Target = S;
        type Checker = T;
        fn verify<R: DebuggableRng<S, T>>(gen: &mut R, target: &mut Self::Target, checker: &mut Self::Checker) -> Result<(), QueryFail> {
            LengthQuery::verify(gen, target, checker)?;
            let vec: Vec<_> = (0..N::N).map(|_| V::generate(gen)).collect();
            *target = S::from(vec.clone());
            *checker = T::from(vec.clone());
            Ok(())
        }
}
