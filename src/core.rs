pub trait Named: Sized {
    fn name() -> String;
}

impl Named for rand_xoshiro::Xoshiro128Plus {
    fn name() -> String { format!("Xorshiro128+") }
}

pub struct QueryFail {
    pub fail_query: String,
    pub fail_detail: String,
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
