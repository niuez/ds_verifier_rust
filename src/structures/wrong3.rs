use crate::core::Named;
use crate::query::{ Length, AccessAt, UpdateAt };
use crate::data::Data;
use crate::structures::Structure;

pub struct Wrong3<T>(Vec<T>);

impl<T> From<Vec<T>> for Wrong3<T> {
    fn from(vec: Vec<T>) -> Self { Wrong3(vec) }
}

impl<T: Named> Structure for Wrong3<T> {
    fn new() -> Self { Wrong3(Vec::new()) }
}

impl<T: Named> Named for Wrong3<T> {
    fn name() -> String { format!("std::vec::Vec<{}>(Wrong at 3)", T::name()) }
}

impl<T> Length for Wrong3<T> {
    fn length(&self) -> usize { self.0.len() }
}

impl<T: Data> AccessAt for Wrong3<T> {
    type Type = T;
    fn access_at(&self, i: usize) -> T {
        if i == 3 { self.0[0].clone() }
        else  { self.0[i].clone() }
    }
}

impl<T: Data> UpdateAt for Wrong3<T> {
    type Type = T;
    fn update_at(&mut self, i: usize, val: &T) { self.0[i] = val.clone(); }
}

define_number!(N100, 100);

#[test]
fn wrong3() {
    use crate::query::util::Times;
    use crate::query::init::InitByVec;
    use crate::query::{ AccessAtQuery };
    use crate::verify::Verify;
    use rand_xoshiro::Xoshiro128Plus;
    use crate::structures::wrong3::Wrong3;
    type Fp = crate::data::Mod998244353;
    type T = Wrong3<Fp>;
    type C = Vec<Fp>;

    type VecVerify =
        Verify<
            Xoshiro128Plus,
            process! {
                InitByVec<T, C, Fp, N100>,
                Times<N100, AccessAtQuery<T, C>>
            }
        >;
    let mut result = VecVerify::new(0, 5);
    result.verify();
    result.panic_by_fail();
}

#[test]
fn wrong3_debug() {
    use crate::query::util::Times;
    use crate::query::init::InitByVec;
    use crate::query::{ AccessAtQuery };
    use crate::verify::Verify;
    use rand_xoshiro::Xoshiro128Plus;
    use crate::structures::wrong3::Wrong3;
    type Fp = crate::data::Mod998244353;
    type T = Wrong3<Fp>;
    type C = Vec<Fp>;

    type VecVerify =
        Verify<
            Xoshiro128Plus,
            process! {
                InitByVec<T, C, Fp, N100>,
                Times<N100, AccessAtQuery<T, C>>
            }
        >;
    let seed = 0;
    let trace_count = 37;
    let tracer = |w3: &T, _: &C| {
        println!("w3 = {:?}", w3.0);
    };
    let mut result = VecVerify::debug(seed, trace_count..trace_count+1, tracer);
    result.verify();
    result.panic_by_fail();
}
