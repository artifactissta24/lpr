use std::ptr;
    fn print_var(v: u8) {
    println!("{v}");
}
    unsafe fn fn12_rs() -> ([u128; 7], *mut i8, *mut bool) {
    let mut v2       = false;
    let mut v8         ;
    let mut v9        = 0;
    let mut v12                           ;
    let mut v17                             ;
    let mut v20: [u8; 8] = Default::default();
    let mut v21                              ;
    let mut v31: (bool, u8            ) = Default::default();
    let mut v33: ([u128; 7], *mut i8, *mut bool) = ([0; 7], ptr::null_mut(), ptr::null_mut());
    let mut v39: (usize, [u128; 7], ([u32; 6], usize, *mut  u32    ), [u32; 2]) =
        (0, [0; 7], ([0; 6], 0, ptr::null_mut()), [0; 2]);
    let mut ret                                  = ([0; 7], ptr::null_mut(), ptr::null_mut());
    ret.2 =       ptr::addr_of_mut!(v2);
    'l0: loop {
        v12 =       ptr::addr_of_mut!(v20[v9]);
        v20 = [197_u8; 8];
        v9 = 2_usize;
             loop {
            match *v12 {
                197 => {
                    v8 = 13978819448286864680_u64;
                    v33.2 = ret.2;
                    match v39.0 {
                        0 =>
                            'l2: loop {
                                 *v12  = 22;
                                     loop {
                                    v21 = v20;
                                    match v8 {
                                        13978819448286864680 => {
                                            v39.2 .0 = [2262110980_u32; 6];
                                            v8 =  13152832795211590855_u64;
                                            v39.0 = 6;
                                            v17 = v33.2;
                                            v33.2 =       ptr::addr_of_mut!(v31.0);
                                            v31.1 = *v12;
                                             *v17  = true;
                                            v20 = v21;
                                            match v39.0 {
                                                6 => {
                                                    print_var(v31.1);
                                                }
                                                0 => continue 'l2,
                                                _ => return ret
                                            }
                                        }
                                        _ => continue 'l0
                                    }
                                }
                            }
                        _ => return ret
                    }
                }
                4 => {
                    v12 =       ptr::addr_of_mut!(v20[v9]);
                }
                _ => return ret
            }
        }
    }
}
    fn main() {
    unsafe {
        fn12_rs();
    }
}
