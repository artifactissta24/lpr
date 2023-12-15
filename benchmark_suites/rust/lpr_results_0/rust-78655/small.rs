const a: *const u32 = {
    let b;
    &b
};
fn c() {
    let a = a;
}
