trait TypeVal<T> {
    const C: T;
}
struct Five;
struct B<F> {
    h: G,
}
impl<F: TypeVal<C>> TypeVal<usize> for B<F> {}
fn main() {
    [1; <B<Five>>::C];
}
