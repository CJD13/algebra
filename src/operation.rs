pub trait O2<S> {
    const F: fn(S, &S) -> S;
}
pub trait O0<S> {
    const F: fn() -> S;
}
pub trait O1<S> {
    const F: fn(S) -> S;
}
//Operations on Cartesian products
impl<S,T,O:O2<S>,P:O2<T>> O2<(S,T)> for (O,P) {
    const F: fn((S,T), &(S,T)) -> (S,T) = |(s1,t1) , (s2,t2)| (O::F(s1,s2),P::F(t1,t2));
}

