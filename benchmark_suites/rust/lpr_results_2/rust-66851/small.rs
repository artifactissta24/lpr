fn main() {
    enum J {
        K(u64),
        M,
    }
    fn n(_p: u64, _q: u64, ad: &[i32]) -> J {
        if let Ok(r) = "20313839404245".parse() {
            J::K(r)
        } else {
            J::M
        }
    }
    let result = if let J::K(s) = n(4096u64, 4096u64, &[]) {
        Some(s)
    } else {
        None
    };
    assert_eq!(result, Some(20313839404245));
}
