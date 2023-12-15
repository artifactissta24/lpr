extern crate core;
use core::ptr;
    fn dump_var(val0: u8 ) {
    println!("{val0}");
}
    unsafe fn fn10_rs() -> *const *mut isize{
    let mut _16           ;
    let mut isize_min           ;
    let mut _46         ;
    let mut _47         ;
    let mut _88           ;
    let mut _31                           ;
    let mut _33                                 ;
    let mut _90                           ;
    let mut _177           = ptr::null_mut();
    let mut _44: ((u8 , u8 , u8),_     ,f32)= Default::default();
    let mut tup: (((u8 , u8 , u8), isize,f32),    ) = Default::default();
    let     _95                                     = Default::default();
        _31 =       ptr::addr_of_mut!(_44.0 .2);
             *_31|=  1;
            isize_min = isize::MIN;
            'bb65: loop {
                let tup_ptr =       ptr::addr_of_mut!(tup);
                _31 =       ptr::addr_of_mut!((*tup_ptr).0 .0 .2);
                _16 = 18215089233857363959_usize;
                        _90 = _31.wrapping_offset(isize::MIN);
                        tup.0 = _44;
                        _46 = 42;
                        _88 = _16;
                            _31 = _90.wrapping_offset(isize_min);
                                             loop {
                                     *tup_ptr   = _95;
                                    isize_min = _46          ;
                                    _47 =        *_31;
                                     *tup_ptr  = _95;
                                    match _88 {
                                        18215089233857363959 => {
                                            _88 =             0            ;
                                            _33 =       ptr::addr_of_mut!(_177);
                                            match _46      {
                                                42 => {
                                                     *_33  =       ptr::addr_of_mut!(_46);
                                                            dump_var(_47);
                                                            return       ptr::addr_of_mut!(_177);
                                                }
                                                    _ => continue
                                            }
                                        }
                                        0 => continue      ,
                                        _ => continue 'bb65
                                    }
                                }
            }
}
    fn main() {
    unsafe {
        fn10_rs();
    }
}
