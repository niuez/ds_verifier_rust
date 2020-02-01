use crate::debug::RngStatus;

pub trait Named: Sized {
    fn name() -> String;
}

impl Named for rand_xoshiro::Xoshiro128Plus {
    fn name() -> String { format!("Xorshiro128+") }
}

pub struct QueryFail {
    pub query_name: String,
    pub detail: String,
    pub trace_cnt: usize,
    pub seed: u64,
}

impl QueryFail {
    pub fn new<R: RngStatus>(query_name: String, detail: String, gen: &R) -> Self {
        QueryFail {
            query_name, detail,
            trace_cnt: gen.trace_count(),
            seed: gen.seed(),
        }
    }
}

pub trait Number {
    const N: usize;
}

#[macro_export]
macro_rules! define_number {
    ($st: ident, $n: expr) => {
        pub struct $st {}
        impl crate::core::Number for $st { const N: usize = $n;  }
    }
}
