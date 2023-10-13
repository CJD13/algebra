pub trait O2<S> {
    const F: fn(S, S) -> S;
}
pub trait O0<S> {
    const F: fn() -> S;
}
pub trait O1<S> {
    const F: fn(S) -> S;
}
pub struct i64Plus {}
impl O2<i64> for i64Plus {
    const F: fn(i64, i64) -> i64 = |a, b| a + b;
}
pub struct i64Times {}
impl O2<i64> for i64Times {
    const F: fn(i64, i64) -> i64 = |a, b| a * b;
}
