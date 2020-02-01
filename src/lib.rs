#[macro_use]
pub mod core;
pub mod query;
pub mod data;
pub mod structures;
pub mod verify;
pub mod debug;

define_number!(N100, 100);

#[test]
fn verify() {
    use query::util::Times;
    use query::init::InitByVec;
    use query::{ AccessAtQuery, UpdateAtQuery };
    use verify::Verify;
    use rand_xoshiro::Xoshiro128Plus;
    type Fp = data::Mod998244353;
    type T = Vec<Fp>;
    type C = Vec<Fp>;

    type VecVerify =
        Verify<
            Xoshiro128Plus,
            process! {
                InitByVec<T, C, Fp, N100>,
                InitByVec<T, C, Fp, N100>,
                Times<N100, random_select!(AccessAtQuery<T, C>, AccessAtQuery<T, C>, UpdateAtQuery<T, C>)>
            }
            >;
    let mut result = VecVerify::new(0, 5);
    result.verify();
    println!("{:?}", result);
}
