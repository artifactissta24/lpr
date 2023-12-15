fn main() {
    let mut l = [0, 1, 0, 0];
    let b = &l;
    let mut result = [0; 4];
    result[0] = b[1];
    l = result;
    println!("{:?}", l);
}
