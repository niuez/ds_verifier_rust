#[macro_use]
pub mod core;
pub mod query;
pub mod data;
pub mod structures;
pub mod verify;

define_number!(N100, 100);

#[test]
fn verify() {
    use query::util::{ Times, Process };
    use query::init::InitByVec;
    use query::AccessAtQuery;
    use verify::Verify;
    use rand_xoshiro::Xoshiro128Plus;
    type Fp = data::Mod998244353;
    type T = Vec<Fp>;
    type C = Vec<Fp>;

    type VecVerify =
        Verify<
            Xoshiro128Plus,
            Process<InitByVec<T, C, Fp, N100>, Times<N100, AccessAtQuery<T, C>>>
            >;
    let result = VecVerify::verify();
    println!("{:?}", result);
}
