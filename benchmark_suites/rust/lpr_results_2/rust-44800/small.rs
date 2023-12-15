use std::collections::VecDeque;
fn main() {
    let mut k = VecDeque::with_capacity(32);
    let mut g_data = vec![0x45, 20, 0b1000_10_00, 0b1000_10_00];
    let hw = 0b1000_10_00;
    let addr: u8 = 1;
    k.push_front(3);
    k.push_front(2);
    k.push_front(1);
    k.push_front(g_data[0]);
    g_data.push(hw);
    g_data.push(hw);
    for _ in 0..18 {
        g_data.push(addr);
    }
    g_data.push(55);
    g_data.push(55);
    g_data.push(hw);
    g_data.push(hw);
    k.reserve_exact(g_data.len());
    println!();
    k.extend(&[0xD9]);
}
