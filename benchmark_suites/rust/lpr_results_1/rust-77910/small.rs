fn a(b: &i32) -> &i32 {
    b
}
fn d() {
    assert_eq!(a, a);
}
