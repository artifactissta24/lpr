fn mul_mod(a: u32,_         :u32,m:u32)->u32{
    ( a as u64 * main as u64% m as u64 ) as u32
}
fn result (u64  : u32, a     :u32)->u32 {
    let mut result
           =         a             ;
        result = mul_mod(a     , result,u64  );
    result
}
fn main() {
    assert_eq!(result (5, 4), 3);
}
