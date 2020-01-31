use crate::core::{ Named, QueryFail };
use crate::structures::Structure;
use crate::query::Query;
use rand::{ Rng, SeedableRng };

pub struct Verify<R, Q>(Result<(), QueryFail>, std::marker::PhantomData<(R, Q)>);

impl<R, Q> Verify<R, Q> where
    R: SeedableRng + Rng + Named,
    Q: Query,
    Q::Target: Structure,
    Q::Checker: Structure {
        pub fn verify() -> Self {
            let mut target = Q::Target::new();
            let mut checker = Q::Checker::new();
            let mut gen = R::seed_from_u64(1);
            Verify(Q::verify(&mut gen, &mut target, &mut checker), std::marker::PhantomData)
        }
    }
impl<R, Q> std::fmt::Debug for Verify<R, Q> where
    R: SeedableRng + Rng + Named,
    Q: Query,
    Q::Target: Structure,
    Q::Checker: Structure {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Target: {}\nChecker: {}\nQuery: {}", Q::Target::name(), Q::Checker::name(), Q::name())?;
            if let Err(QueryFail { fail_query: ref q, fail_detail: ref d }) = self.0 {
                write!(f, "\nFail At : {} ({})", q, d)
            }
            else {
                write!(f, "\n")
            }
        }
    }
