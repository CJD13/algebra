trait Outer<S: Inner>
where
    Self: Bound,
    S::T: Bound,
{
}

trait Inner {
    type T;
}
trait Bound {}

#[test]
fn main() {
    println!("test")
}
