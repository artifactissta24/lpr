    fn first() -> bool {
    let mut a = (0., true);
    let b = (0., false);
    let a_ptr_and_b = (core::ptr::addr_of_mut!(a), b);
    let got = unsafe { second(a_ptr_and_b.0, a_ptr_and_b, true, true) };
           got
}
unsafe fn second(
_        : *mut (f32, bool),
    a_ptr_and_b: (*mut (f32, bool), (f32, bool)),
    ptr   :bool,
    f32   : bool
) -> bool {
    let b_bool_ptr = core::ptr::addr_of!(a_ptr_and_b.1 .1) as *mut bool;
     *b_bool_ptr  = f32             ;
                 a_ptr_and_b;
           a_ptr_and_b.1.1
}
    fn main() {
    println!("{}", first());
}
