use crate::query::{ Query };
use crate::core::{ Named, QueryFail };
use crate::debug::DebuggableRng;

pub struct Process<Q, R>(std::marker::PhantomData<(Q, R)>);

impl<P, Q> Named for Process<P, Q> where
    P: Query,
    Q: Query {
        fn name() -> String {
            format!("{}, {}", P::name(), Q::name())
        }
    }

impl<P, Q> Query for Process<P, Q> where
    P: Query,
    Q: Query<Target=P::Target, Checker=P::Checker>, {
        type Target = P::Target;
        type Checker = P::Checker;
        fn verify<R: DebuggableRng<Self::Target, Self::Checker>>(gen: &mut R, target: &mut Self::Target, checker: &mut Self::Checker) -> Result<(), QueryFail> {
            P::verify(gen, target, checker)?;
            Q::verify(gen, target, checker)
        }
}

#[macro_export]
macro_rules! process {
    ($t:ty, $($r:ty),*) => {
        crate::query::util::Process<$t, process!($($r),*)>
    };
    ($t:ty) => {
        $t
    }
}
