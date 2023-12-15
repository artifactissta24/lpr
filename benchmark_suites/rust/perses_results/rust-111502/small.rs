    fn first() -> bool {
    let mut a = (0., true);
    let b = (0., false);
    let a_ptr_and_b = (core::ptr::addr_of_mut!(a), b);
    let got = unsafe { second(a_ptr_and_b.0, a_ptr_and_b, true, true) };
    return got;
}
unsafe fn second(
    a_ptr: *mut (f32, bool),
    a_ptr_and_b: (*mut (f32, bool), (f64, bool)),
    t: bool,
    t_copy: bool
) -> bool {
    let b_bool_ptr = core::ptr::addr_of!(a_ptr_and_b.1 .1) as *mut bool;
    let t_or_t = t         ;
     *b_bool_ptr  = t_or_t          ;
                 a_ptr_and_b;
    return a_ptr_and_b.1.1              ;
}
    fn main() {
    println!("{}", first());
}
