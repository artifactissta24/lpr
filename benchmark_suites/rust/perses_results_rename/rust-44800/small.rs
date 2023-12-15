use std::collections::VecDeque;
struct a {
    c: VecDeque<u8>,
}
struct d {
    data: Vec<u8>,
}
impl a {
    fn e() -> Self {
        let c = VecDeque::with_capacity(32);
        a { c }
    }
    fn f(&mut self, g: &d) {
        self.c.reserve(g.data.len());
        for b in g.data.iter() {
            self.c.push_front(*b);
        }
    }
    fn h(&mut self, data: &[u8]) {
        self.c.extend(data);
    }
}
impl d {
    fn with_capacity(capacity: usize) -> Self {
        let data = Vec::with_capacity(capacity);
        d { data }
    }
    fn i(&mut self, data: u8) {
        self.data.push(data);
    }
    fn m(&mut self, data: u16) {
        self.data.push(data as u8);
        self.data.push(data as u8);
    }
}
fn j(k: &mut a) {
    let mut g = d::with_capacity(20);
    let b = 0x45;
    g.i(b);
    g.i(20);
    let hw = 20;
    g.m(hw);
    g.m(21);
    g.m(23);
    k.f(&g);
}
fn l(k: &mut a) {
    let mut g = d::with_capacity(30);
    let hw = 0b1000_10_00;
    g.m(hw);
    let addr = [1, 2, 3, 4, 5, 6];
    for &b in addr.iter() {
        g.i(b);
    }
    for &b in addr.iter() {
        g.i(b);
    }
    for &b in addr.iter() {
        g.i(b);
    }
    g.m(55);
    g.m(hw);
    k.f(&g);
    let fcs = [0xD9];
    println!();
    k.h(&fcs);
}
fn main() {
    let mut k = a::e();
    j(&mut k);
    l(&mut k);
}
