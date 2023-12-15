fn c() -> bool {
    let a_3 = false;
    unsafe { f((core::ptr::null_mut(), 0.0, a_3), true) }
}
unsafe fn f(a: (*mut f32, f64, bool), b: bool) -> bool {
    let i = core::ptr::addr_of!(a.2) as *mut bool;
    *i = b;
    a;
    a.2
}
fn main() {
    println!("{}", c());
}
