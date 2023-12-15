#![feature(const_hash)]
use std::collections::hash_map::DefaultHasher;
use std::hash::*             ;
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
    fn fn0(    _2: [isize; 7],     _3: usize,     _4: i32,_         :u64)             {
    let ret             = [13; 2];
    let _6 = fn1(_2);
    fn5(ret, _2, _3, _2, _2, _4, _6);
    dump_var(true     , _2   , 0,  4     );
    dump_var((), (), _2 , _2 );
}
    fn fn1(_      : [isize; 7]) -> *mut isize {
    let arr = [0   ];
    let mut _21                                   = (false,0,arr,0  );
    dump_var(_21, _21, 0, 0);
    unsafe {
        _21.hash(&mut H);
    }
                 ptr::addr_of_mut!(_21.3)
}
fn fn5(
        _2: [isize; 2],
        _6: [isize; 7],
_           :usize,
    mut _9: [isize; 7],
_          : [isize; 7],
_            :i32,
        _16: *mut isize
) {
    let mut _23: (
        *const usize,
        (usize, isize),
        (char, isize,(isize,),isize),
        char,
        (
            (f64, i64, i8),
            [f64; 1],
            (usize, isize),
            isize,
            (isize, i64),
            [f64; 1]
        ),
        *const isize
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
        let     _21 =  0  ;
            _23.0 =       ptr::addr_of_mut!(_23.1 .0);
            _23.4 .0 = (f64::NAN,   1102345069964335552_i64 , 9_i8);
            dump_var(0, 0, _6,     0   );
            _27.0 = _23.4 .0;
            match _27.0 .2 {
                9 => {
                    dump_var(_9, _21, _2 , _2 );
                    dump_var(_2, _21, _6, _2);
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
