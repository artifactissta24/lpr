use primal_sieve::Primes;
use std::ops::{Add, MulAssign};
fn main() {
    let mut c = d(1);
    Primes::all()
        .take_while(|&e| e < 4)
        .for_each(|e| c *= d(e as u32).pow(8 / e as u32) + d(1));
    assert_eq!(c.0, 650);
}
#[derive(Clone, Copy)]
struct d(u32);
impl d {
    const g: u32 = 1000000000;
}
impl Add for d {
    type Output = d;
    fn add(mut self, h: d) -> d {
        self.0 += h.0;
        self
    }
}
impl MulAssign for d {
    fn mul_assign(&mut self, h: d) {
        unsafe {
            core::arch::asm!(
                "mul edx",
                "div {:e}",
                in(reg) Self::g,
                inout("eax") h.0 => _,
                inout("edx") self.0
            );
        }
    }
}
impl d {
    fn pow(self, mut exp: u32) -> Self {
        let mut base = self;
        base *= base;
        exp >>= 1;
        let mut acc = base;
        exp >>= 1;
        base *= base;
        if exp == 1 {
            acc *= base;
        }
        acc
    }
}
