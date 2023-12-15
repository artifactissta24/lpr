const a: isize = 10;
const b: &() = std::mem::transmute(a);
fn c() {
    match &() {
        b => 9,
    };
}
