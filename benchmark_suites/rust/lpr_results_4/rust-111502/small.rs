fn c() -> bool {
    unsafe { f((core::ptr::null_mut(), (1.0, false))) }
}
unsafe fn f(d: (*mut f64, (f64, bool))) -> bool {
    let i = core::ptr::addr_of!(d.1 .1) as *mut bool;
    *i = true;
    d;
    return d.1 .1;
}
fn main() {
    println!("{}", c());
}
