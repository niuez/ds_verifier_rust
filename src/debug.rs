use rand::{ Rng, RngCore, SeedableRng };

pub trait RngStatus: Rng {
    fn trace_count(&self) -> usize;
    fn seed(&self) -> u64;
}

pub trait DebuggableRng<T, C>: RngStatus {
    fn debugtrace(&mut self, target: &T, checker: &C);
}

pub struct DebugRng<Target, Checker, R, Ran> {
    trace: fn(&Target, &Checker),
    gen: R,
    cnt: usize,
    seed: u64,
    debug_range: Option<Ran>,
    _p: std::marker::PhantomData<(Target, Checker)>,
}
impl<T, C, R, Ran> DebugRng<T, C, R, Ran> where
    R: Rng + SeedableRng, Ran: std::ops::RangeBounds<usize> {
        pub fn new(seed: u64) -> Self {
            DebugRng {
                trace: |_: &T, _: &C| {},
                gen: R::seed_from_u64(seed),
                cnt: 0,
                seed,
                debug_range: None,
                _p: std::marker::PhantomData,
            }
        }
        pub fn debug_new(seed: u64, ran: Ran, trace_f: fn(&T, &C)) -> Self {
            DebugRng {
                trace: trace_f,
                gen: R::seed_from_u64(seed),
                cnt: 0,
                seed,
                debug_range: Some(ran),
                _p: std::marker::PhantomData,
            }
        }
        pub fn debugtrace(&mut self, target: &T, checker: &C) {
            self.cnt += 1;
            if let Some(ref ran) = self.debug_range {
                if ran.contains(&self.cnt) {
                    (self.trace)(target, checker);
                }
            }
        }
    }

impl<T, C, R, Ran> RngStatus for DebugRng<T, C, R, Ran> where
  R: Rng + SeedableRng, Ran: std::ops::RangeBounds<usize> {
      fn trace_count(&self) -> usize {
          self.cnt
      }
      fn seed(&self) -> u64 {
          self.seed
      }
  }
impl<T, C, R, Ran> DebuggableRng<T, C> for DebugRng<T, C, R, Ran> where
    R: Rng + SeedableRng, Ran: std::ops::RangeBounds<usize> {
        fn debugtrace(&mut self, target: &T, checker: &C) {
            self.cnt += 1;
            if let Some(ref ran) = self.debug_range {
                if ran.contains(&self.cnt) {
                    (self.trace)(target, checker);
                }
            }
        }
    }
impl<T, C, R, Ran> RngCore for DebugRng<T, C, R, Ran> where
    R: Rng + SeedableRng {
        fn next_u32(&mut self) -> u32 { self.gen.next_u32() }
        fn next_u64(&mut self) -> u64 { self.gen.next_u64() }
        fn fill_bytes(&mut self, dest: &mut [u8]) { self.gen.fill_bytes(dest) }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> { self.gen.try_fill_bytes(dest) }
    }
