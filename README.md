# ds\_verifier for rust

To verify data strucutures written in Rust lang

## How To Use

(以下の実装は`src/structures/wrong3.rs`にあります.)

添字`3`にアクセスしたときに添字`0`にアクセスしてしまう配列のデータ構造`Wrong3`を実装するとします.

```rust
use crate::core::Named;
use crate::query::{ Length, AccessAt };
use crate::data::Data;
use crate::structures::Structure;

pub struct Wrong3<T>(Vec<T>);

// 初期化
impl<T: Named> Structure for Wrong3<T> {
    fn new() -> Self { Wrong3(Vec::new()) }
}

// データ構造の名前
impl<T: Named> Named for Wrong3<T> {
    fn name() -> String { format!("std::vec::Vec<{}>(Wrong at 3)", T::name()) }
}

// 配列で初期化する
impl<T> From<Vec<T>> for Wrong3<T> {
    fn from(vec: Vec<T>) -> Self { Wrong3(vec) }
}

// 配列の長さを返すクエリ
impl<T> Length for Wrong3<T> {
    fn length(&self) -> usize { self.0.len() }
}

// 配列にアクセスして値を返すクエリ
impl<T: Data> AccessAt for Wrong3<T> {
    type Type = T;
    fn access_at(&self, i: usize) -> T {
        // i = 3にアクセスしたときにバグる
        if i == 3 { self.0[0].clone() }
        else  { self.0[i].clone() }
    }
}
```

要素の型`ModInt 998244353`, 長さ`100`の`Vec`で初期化して, `100`回ランダムにアクセスするクエリをverifyしてみます.  
上のクエリを正しく処理することのできる`Vec`をCheckerとします.

```rust
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
                InitByVec<T, C, Fp, N100>        // 長さ100のVecで初期化
                Times<N100, AccessAtQuery<T, C>> // 100回ランダムにアクセス
            }
        >;
    let mut result = VecVerify::new(0, 5);
    result.verify();
    result.panic_by_fail();
}
```

testをすると,

```
---- structures::wrong3::wrong3 stdout ----
thread 'structures::wrong3::wrong3' panicked at '
=== Fail ===
Target: std::vec::Vec<ModInt[998244353]>(Wrong at 3)
Checker: std::vec::Vec<ModInt[998244353]>
Query: Init by Vec[ModInt[998244353]; 100], [AccessAt] for times 100
Fail At : AccessAt (access_at 3: target results m(721274730) but checker results m(8940
55170))
trace_count: 37
seed: 0
============
```

`access_at 3`でバグが発生していることがわかります.

このクエリを処理する直前に何が起こっていたのかをデバッグしてみます.

```rust
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
        // wrong3の中身を表示
        println!("w3 = {:?}", w3.0);
    };
    let mut result = VecVerify::debug(seed, trace_count..trace_count+1, tracer);
    result.verify();
    result.panic_by_fail();
}
```

これをテストすると, 以下のように, `trace_count`の付近での`Wrong3`の状態をデバッグすることができます. 実際に, `w3[3] = m(894055170)`であるのに出力が間違っていることから, インデックスのバグであると予想することができます.

```
---- structures::wrong3::wrong3_debug stdout ----
w3 = [m(721274730), m(222418339), m(232663824), m(894055170), m(958827451), m(403499650
), m(967714203), m(778584895), m(453571625), m(379738777), m(159010686), m(103018920),
m(400760644), m(258238183), m(818419952), m(41081635), m(915754895), m(359791315), m(81
4668035), m(34965562), m(783420553), m(311495662), m(899747381), m(267020439), m(880449
140), m(91621721), m(595499886), m(24180373), m(390395257), m(348417156), m(959320716),
 m(77720604), m(135737809), m(278076062), m(187625852), m(603946556), m(698354901), m(4
15373758), m(251997715), m(6042184), m(40327379), m(662241018), m(951637644), m(7756823
23), m(617522763), m(565043471), m(307005034), m(283209056), m(372366731), m(543854063)
, m(841665453), m(438048833), m(139490710), m(170025660), m(932505504), m(231943078), m
(437127340), m(923925711), m(880291025), m(845677979), m(923707370), m(868927517), m(48
6888033), m(92703344), m(50036474), m(506788384), m(154149926), m(360757915), m(7767105
30), m(340807623), m(131377692), m(150633702), m(625856817), m(739778670), m(47032396),
 m(775453658), m(698402762), m(71312455), m(29829692), m(130943700), m(481339657), m(55
0673827), m(77368765), m(109010959), m(643011620), m(16182099), m(125967218), m(3158343
23), m(458614756), m(865070445), m(799511255), m(647596520), m(592101319), m(561757452)
, m(632184849), m(784286197), m(155563874), m(455736933), m(397142649), m(449830049)]
thread 'structures::wrong3::wrong3_debug' panicked at '
=== Fail ===
Target: std::vec::Vec<ModInt[998244353]>(Wrong at 3)
Checker: std::vec::Vec<ModInt[998244353]>
Query: Init by Vec[ModInt[998244353]; 100], [AccessAt] for times 100
Fail At : AccessAt (access_at 3: target results m(721274730) but checker results m(8940
55170))
trace_count: 37
seed: 0
============
```


