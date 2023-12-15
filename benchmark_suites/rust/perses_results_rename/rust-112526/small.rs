extern crate core;
use core::ptr;
fn dump_var(val0: u16) {
    println!("{val0}");
}
unsafe fn a() -> *const *mut i64 {
    let mut _16;
    let mut isize_min;
    let mut _46;
    let _107 = 0;
    let mut _47;
    let mut _88;
    let mut _31;
    let mut _33;
    let mut _90;
    let mut _177 = ptr::null_mut();
    let mut _44: ((u64, u64, u8), u32, f32) = Default::default();
    let mut tup: (((u64, u64, u8), u32, f32), bool) = Default::default();
    let mut _95: (((u64, u64, u8), u32, f32), bool) = Default::default();
    let two = 2;
    _31 = ptr::addr_of_mut!(_44.0 .2);
    *_31 = 1;
    isize_min = isize::MIN;
    'bb65: loop {
        let tup_ptr = ptr::addr_of_mut!(tup);
        _31 = ptr::addr_of_mut!((*tup_ptr).0 .0 .2);
        _16 = 18215089233857363959_usize;
        _90 = _31.wrapping_offset(isize::MIN);
        _95.0 = _44;
        _46 = 42;
        _88 = _16;
        _31 = _90.wrapping_offset(isize_min);
        loop {
            let tup_ptr2 = ptr::addr_of_mut!(tup);
            *tup_ptr2 = _95;
            isize_min = _107;
            _47 = two >> *_31;
            *tup_ptr = _95;
            match _88 {
                18215089233857363959 => {
                    _88 = 0;
                    _33 = ptr::addr_of_mut!(_177);
                    match _46 {
                        42 => {
                            *_33 = ptr::addr_of_mut!(_46);
                            dump_var(_47);
                            return ptr::addr_of!(_177);
                        }
                        _ => continue,
                    }
                }
                0 => continue,
                _ => continue 'bb65,
            }
        }
    }
}
fn main() {
    unsafe {
        a();
    }
}
