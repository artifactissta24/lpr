use primal_sieve::Primes;
use std::ops::{Add, MulAssign};
fn main() {
    assert_eq!(a(4), 650);
}
fn a(b: u32) -> u32 {
    let mut c = d(1);
    Primes::all()
        .take_while(|&e| e < b as usize)
        .for_each(|e| c *= d(e as u32).pow(2 * (b / e as u32)) + d(1));
    c.0
}
#[derive(Clone, Copy)]
struct d(u32);
impl d {
    const g: u32 = 10u32.pow(9);
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
            core::arch::asm!
            (
                "mul edx",
                "div {:e}",
                in(reg) Self::g,
                inout("eax") h.0 => _,
                inout("edx") self.0
            );
        }
    }
}
