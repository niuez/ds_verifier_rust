use crate::query::{ Query };
use crate::core::{ Named, QueryFail };
use crate::debug::DebuggableRng;

pub struct RandomSelect<Q, R>(std::marker::PhantomData<(Q, R)>);

impl<Q, R> Named for RandomSelect<Q, R> where
    Q: Query,
    R: Query {
        fn name() -> String {
            format!("{} or {}", Q::name(), R::name())
        }
    }

pub trait QuerySize {
    fn size() -> usize;
}

impl<T, C> QuerySize for (T, C) {
    fn size() -> usize { 0 }
}

impl<Q, R: QuerySize> QuerySize for RandomSelect<Q, R> {
    fn size() -> usize { 1 + R::size() }
}

impl<P, Q> Query for RandomSelect<P, Q> where
    P: Query,
    Q: Query<Target=P::Target, Checker=P::Checker> + QuerySize, {
        type Target = P::Target;
        type Checker = P::Checker;
        fn verify<R: DebuggableRng<Self::Target, Self::Checker>>(gen: &mut R, target: &mut Self::Target, checker: &mut Self::Checker) -> Result<(), QueryFail> {
            if gen.gen_range(0, Self::size()) == 0 {
                P::verify(gen, target, checker)
            }
            else {
                Q::verify(gen, target, checker)
            }
        }
    }

#[macro_export]
macro_rules! random_select {
    ($t:ty $(,$r:ty)*) => {
        random_select_raw!(<$t as crate::query::Query>::Target, <$t as crate::query::Query>::Checker; $t, $($r)*)
    };
    ($t:ty) => {
        $t
    }
}

#[macro_export]
macro_rules! random_select_raw {
    ($target:ty, $checker:ty; $t:ty) => {
        crate::query::util::RandomSelect<$t, ($target, $checker)>
    };
    ($target:ty, $checker:ty; $t:ty, $($r:ty)*) => {
        crate::query::util::RandomSelect<$t, random_select_raw!($target, $checker; $($r), *)>
    }
}
