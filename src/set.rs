pub trait Subset<T>
where
    T: Set,
{
    /// A subset of `T` is determined by which elements of `T` are members of it.
    fn contains(t: &T) -> bool;
    /// Any object of type `Self` can be coerced into one of type `T`
    fn inclusion(self) -> T;
    /// Some `T`s can be turned into `Self`s
    fn try_from(t: T) -> Self;
}

/// A finite subset can be iterated over. An implementation must guarantee that each member t of T
/// is `==` to exactly one item of the iterator if Self::contains(t) is true and does not == any if
/// it is false.
pub trait FiniteSubset<T>: Subset<T> + IntoIterator<Item = T>
where
    T: Set,
{
    const ORDER: usize;
}
/// A struct implementing FiniteSet must have ORDER equal to the number of objects of that struct.
/// It must also be able to provide an iterator over its elements. Any member of the struct must be
/// == to exactly one of the iterms returned by the iterator.
pub trait FiniteSet: Set + IntoIterator<Item = Self> {
    const ORDER: usize;
}
pub trait Set: Clone + Eq {}

impl<S, T> Set for (S, T)
where
    S: Set,
    T: Set,
{
}
impl Set for i64 {}

/*impl<S,T> FiniteSubset<T> for S where T:FiniteSet, S:Subset<T> {
    const ORDER: usize = T::get_iter().filter(S::contains).collect::<Vec<_>>().len();
    type IterType = Filter<T::IterType,fn(&T)->bool>;
    fn get_iter() -> Self::IterType {
        T::get_iter().filter(S::contains)
    }
}*/
