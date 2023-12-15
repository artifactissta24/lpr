fn mul_mod(a: u32, b: u32, m: u32) -> u32 {
    ((a as u64 * b as u64) %  m as u64 ) as u32
}
fn pow_mod(a: u32, n: u32, p: u32) -> u32 {
    if n     == 0 {
                mul_mod(a, a, p)
    } else {
                a
    }
}
fn residue(prime: u32, arg: u32) -> u32 {
    let     y = 2;
    let     r      = 0;
    while     2 == 0 {
    }
    let mut result            ;
    result =         arg           ;
        let mut t                       ;
        t = 0;
        t |= 1 <<  r         ;
        t = pow_mod(y, t, prime);
        result = mul_mod(result, t, prime);
    result
}
fn main() {
    assert_eq!(residue(5, 4), 3);
}
