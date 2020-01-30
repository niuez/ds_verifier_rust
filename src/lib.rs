type FailInfo = String;

trait QueryType: Sized {
    type Arg;
    type Output: std::cmp::Eq;
    fn verify<Target: Query<Self>, Checker: Query<Self>>(target: &mut Target, checker: &mut Checker) -> Result<(), FailInfo>;
}

trait Query<Q: QueryType> {
    fn query(&mut self, arg: &Q::Arg) -> Q::Output;
}


struct AccessAt<T>(std::marker::PhantomData<T>);
struct UpdateAt<T>(std::marker::PhantomData<T>);

impl<T: std::cmp::Eq> QueryType for AccessAt<T> {
    type Arg = usize;
    type Output = T;
    fn verify<Target: Query<Self>, Checker: Query<Self>>(target: &mut Target, checker: &mut Checker) -> Result<(), FailInfo> {
        let i = 0;
        let t_res = target.query(&i);
        let c_res = checker.query(&i);
        if t_res == c_res { Ok(()) }
        else { Err("fail".to_string()) }
    }
}

impl<T: std::default::Default> QueryType for UpdateAt<T> {
    type Arg = (usize, T);
    type Output = bool;
    fn verify<Target: Query<Self>, Checker: Query<Self>>(target: &mut Target, checker: &mut Checker) -> Result<(), FailInfo> {
        let i = 0;
        let value = T::default();
        let arg = (i, value);
        let t_res = target.query(&arg);
        let c_res = checker.query(&arg);
        if t_res == c_res { Ok(()) }
        else { Err("fail".to_string()) }
    }
}

impl<T: std::cmp::Eq + Clone> Query<AccessAt<T>> for Vec<T> {
    fn query(&mut self, arg: &usize) -> T { self[*arg].clone() }
}
impl<T: std::cmp::Eq + Clone + std::default::Default> Query<UpdateAt<T>> for Vec<T> {
    fn query(&mut self, arg: &(usize, T)) -> bool { self[arg.0] = arg.1.clone(); true }
}
impl<T: std::cmp::Eq + Clone> Query<AccessAt<T>> for [T; 1] {
    fn query(&mut self, arg: &usize) -> T { self[*arg].clone() }
}
impl<T: std::cmp::Eq + Clone + std::default::Default> Query<UpdateAt<T>> for [T; 1] {
    fn query(&mut self, arg: &(usize, T)) -> bool { self[arg.0] = arg.1.clone(); true }
}

#[test]
fn access_at_test() {
    let mut vec = vec![1];
    let mut arr =     [1];

    match AccessAt::verify(&mut vec, &mut arr) {
        Ok(()) => {}
        Err(message) => panic!(message),
    }

    UpdateAt::verify(&mut vec, &mut arr).unwrap();

    match AccessAt::verify(&mut vec, &mut arr) {
        Ok(()) => {}
        Err(message) => panic!(message),
    }

    vec[0] = 10000;

    match AccessAt::verify(&mut vec, &mut arr) {
        Ok(()) => panic!("should be not equal"),
        Err(_) => {}
    }
}
