fn main() {
    fn f(b: &[i64]) -> [i64; 16] {
        let mut l = [0; 16];
        l[0] = b[0];
        l
    }
    let mut l = [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1];
    l = f(&l);
    println!("{:?}", l);
}
