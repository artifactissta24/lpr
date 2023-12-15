fn c(a: u32, b: u32, d: u32) -> u32 {
    ((a as u64 * b as u64) % d as u64) as u32
}
fn e(a: u32, f: u32, m: u32) -> u32 {
    if f == 0 {
        c(a, a, m)
    } else {
        a
    }
}
fn g(h: u32, arg: u32) -> u32 {
    let i = 2;
    let j = 0;
    while 2 == 0 {}
    let mut k;
    k = arg;
    let mut l;
    l = 0;
    l |= 1 << j;
    l = e(i, l, h);
    k = c(k, l, h);
    k
}
fn main() {
    assert_eq!(g(5, 4), 3);
}
