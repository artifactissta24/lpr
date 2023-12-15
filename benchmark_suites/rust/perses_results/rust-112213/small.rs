#![feature(const_hash)]
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ptr;
static mut H: DefaultHasher = DefaultHasher::new();
fn dump_var<T: Hash, U: Hash, V: Hash, W: Hash>(val0: T, val1: U, val2: V, val3: W) {
    unsafe {
        val0.hash(&mut H);
        val1.hash(&mut H);
        val2.hash(&mut H);
        val3.hash(&mut H);
    }
}
    fn fn0(    _2: [isize; 7],     _3: usize,     _4: i32,     _5: u64) -> [isize; 2] {
    let     _13               =          1   ;
    let     _14             = [0; 6];
    let     _29             = [0; 6];
    let ret             = [13; 2];
    let _6 = fn1(_2);
    fn5(ret, _2, _3, _2, _2, _4, _6);
    dump_var(_4, _5, 0, 0);
    dump_var([ 112; 6], _13  , 0,  4     );
    dump_var((), (), _29, _14);
    return ret;
}
    fn fn1(    _10: [isize; 7]) -> *mut isize {
    let arr = [0; 3];
    let mut _21                                   = (0, 0, arr, 0   );
    let     _24: (u8, u128, [u64; 3], isize, i16) = (0, 0, arr, 0, 0);
    dump_var(_10, _10, 0, 0);
    unsafe {
        _24.hash(&mut H);
        _21.hash(&mut H);
    }
    return       ptr::addr_of_mut!(_21.3);
}
fn fn5(
        _2: [isize; 2],
        _6: [isize; 7],
        _7: usize,
    mut _9: [isize; 7],
        _10: [isize; 7],
        _12: i32,
        _16: *mut isize
) {
    let mut _23: (
        *const usize,
        (usize, u8),
        (char, i32, (i64,), u64),
        char,
        (
            (f64, i64, i8),
            [f32; 1],
            (usize, u8),
            i8,
            (isize, i64),
            [f32; 1]
        ),
        *const u8
    ) = (
        ptr::null(),
        (0, 0),
        ('a', 0, (0,), 0),
        'a',
        ((0., 0, 0), [0.   ], (0, 0), 0, (0, 0), [0.   ]),
        ptr::null()
    );
    let mut _27: ((f64, i64, i8),) = Default::default();
    loop {
        _9 = _6;
        let     _20 =  -9223372036854775808_isize         ;
        let     _21 =  0  ;
            _23.0 =       ptr::addr_of!(_23.1 .0);
            _23.4 .0 = (f64::NAN,   1102345069964335552_i64 , 9_i8);
            dump_var(0, 0, _6,     0   );
            _27.0 = _23.4 .0;
            match _27.0 .2 {
                9 => {
                    dump_var(_9, _21, _20, _12);
                    dump_var(_2, _10, _6, _7);
                    return;
                }
                _ => unsafe {
                     *_16  = 88_isize;
                }
            }
    }
}
    fn main() {
    println!(
        "{:?}",
        fn0(
            [  56_isize ; 7],
            15609822513776909592_usize,
             652623562_i32,
            18399139786288871729_u64
        )
    );
    unsafe {
        println!("hash: {}", H.finish());
    }
}
