extern crate core;
use core::ptr;
unsafe fn a() -> *const *mut i64 {
    let mut isize_min;
    let mut _46;
    let mut _47;
    let mut _88;
    let mut _31;
    let mut _33;
    let mut _90;
    let mut result = ptr::null_mut();
    let mut tup: (((u64, u64, u8), u32, f32), bool) = (((0, 0, 0), 0, 0.0), false);
    _31 = ptr::addr_of_mut!(tup.0 .0 .2);
    *_31 = 1;
    isize_min = isize::MIN;
    'bb65: loop {
        _31 = ptr::addr_of_mut!(tup.0 .0 .2);
        _90 = _31.wrapping_offset(isize::MIN);
        _46 = 42;
        _88 = 18215089233857363959_usize;
        _31 = _90.wrapping_offset(isize_min);
        loop {
            tup = (((0, 0, 0), 0, 0.0), false);
            isize_min = 0;
            _47 = *_31;
            match _88 {
                18215089233857363959 => {
                    _88 = 0;
                    _33 = ptr::addr_of_mut!(result);
                    match _46 {
                        42 => {
                            *_33 = ptr::addr_of_mut!(_46);
                            println!("{_47}");
                            return ptr::addr_of!(result);
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
