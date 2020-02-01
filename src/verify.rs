use crate::core::{ Named, QueryFail };
use crate::structures::Structure;
use crate::query::Query;
use rand::{ Rng, SeedableRng };
use crate::debug::DebugRng;

pub enum VerifyStatus {
    Yet,
    Skipped,
    Done,
    Fail(QueryFail),
}

pub struct Verify<R, Q> {
    seed_s: u64,
    seed_e: u64,
    status: VerifyStatus,
    _p: std::marker::PhantomData<(R, Q)>,
}

impl<R, Q> Verify<R, Q> where
    R: SeedableRng + Rng + Named,
    Q: Query,
    Q::Target: Structure,
    Q::Checker: Structure {
        pub fn new(seed_s: u64, seed_e: u64) -> Self {
            Verify {
                seed_s, seed_e,
                status: VerifyStatus::Yet,
                _p: std::marker::PhantomData,
            }
        }
        pub fn verify(&mut self) {
            for seed in self.seed_s..self.seed_e {
                let mut target = Q::Target::new();
                let mut checker = Q::Checker::new();
                let mut gen = DebugRng::<Q::Target, Q::Checker, R, std::ops::Range<usize>>::new(seed);
                if let Err(fail) = Q::verify(&mut gen, &mut target, &mut checker) {
                    self.status = VerifyStatus::Fail(fail);
                    return;
                }
            }
            self.status = VerifyStatus::Done;
        }
        pub fn debug<Ran: std::ops::RangeBounds<usize>>(seed: u64, debug_ran: Ran, trace_f: fn(&Q::Target, &Q::Checker)) -> Self {
            let mut target = Q::Target::new();
            let mut checker = Q::Checker::new();
            let mut gen = DebugRng::<Q::Target, Q::Checker, R, Ran>::debug_new(seed, debug_ran, trace_f);
            if let Err(fail) = Q::verify(&mut gen, &mut target, &mut checker) {
                Verify {
                    seed_s: seed,
                    seed_e: seed + 1,
                    status: VerifyStatus::Fail(fail),
                    _p: std::marker::PhantomData,
                }
            }
            else {
                Verify {
                    seed_s: seed,
                    seed_e: seed + 1,
                    status: VerifyStatus::Done,
                    _p: std::marker::PhantomData,
                }
            }
        }
    }
impl<R, Q> std::fmt::Debug for Verify<R, Q> where
    R: SeedableRng + Rng + Named,
    Q: Query,
    Q::Target: Structure,
    Q::Checker: Structure {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Target: {}\nChecker: {}\nQuery: {}", Q::Target::name(), Q::Checker::name(), Q::name())?;
            if let VerifyStatus::Fail(QueryFail { fail_query: ref q, fail_detail: ref d }) = self.status {
                write!(f, "\nFail At : {} ({})", q, d)
            }
            else {
                write!(f, "\n")
            }
        }
    }
