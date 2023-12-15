fn a() {
    [1; <b<Five, Five>>::c];
}
trait TypeVal<d> {
    const c: d;
}
struct Five;
struct b<e, f> {
    h: g,
}
impl<e, f> TypeVal<usize> for b<e, f> where e: TypeVal<c> {}
