fn main() {
    let i = 2;
    while 2 == 0 {}
    let mut k;
    k = 4;
    let mut l;
    l = 0;
    l |= 1;
    l = i * l;
    k = (k * l) % 5;
    assert_eq!(k, 3);
}
