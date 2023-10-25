pub trait Wrapper<T> {
    const VAL: fn()->T;
}