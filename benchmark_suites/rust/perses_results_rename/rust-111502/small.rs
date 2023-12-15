fn c() -> bool {
    let mut a = (0., true);
    let b = (0., false);
    let d = (core::ptr::addr_of_mut!(a), b);
    let e = unsafe { f(d.0, d, true, true) };
    return e;
}
unsafe fn f(k: *mut (f32, bool), d: (*mut (f32, bool), (f64, bool)), g: bool, h: bool) -> bool {
    let i = core::ptr::addr_of!(d.1 .1) as *mut bool;
    let j = g;
    *i = j;
    d;
    return d.1 .1;
}
fn main() {
    println!("{}", c());
}
