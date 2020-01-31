use crate::query::{ Query };
use crate::core::{ Named, QueryFail, Number };
use rand::Rng;

pub struct Times<N, Q>(std::marker::PhantomData<(N, Q)>);

impl<N, Q> Named for Times<N, Q> where
    N: Number,
    Q: Query {
        fn name() -> String {
            format!("[{}] for times {}", Q::name(), N::N)
        }
    }

impl<N, Q> Query for Times<N, Q> where
    N: Number,
    Q: Query, {
        type Target = Q::Target;
        type Checker = Q::Checker;
        fn verify<R: Rng>(gen: &mut R, target: &mut Self::Target, checker: &mut Self::Checker) -> Result<(), QueryFail> {
            for _ in 0..N::N {
                Q::verify(gen, target, checker)?;
            }
            Ok(())
        }
}
