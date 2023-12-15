fn main() {
    const N: u64 = 1_000_000_000_000;
    const SIZE: usize = 4;
    type Mat = [[i64; SIZE]; SIZE];
    fn matrix_prod(a: &Mat, b: &Mat, n: usize, m: usize, p: usize) -> Mat {
        let mut res      = Mat::default();
        for i in 0 .. n {
            for j in 0 .. p {
                for k in 0 .. m {
                    res[i][j] =                        b[k][j]             ;
                }
            }
        }
        res
    }
    fn matrix_pow(    a: Mat, s: usize,     n: u64) -> Mat {
        let mut res      = [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]];
                res = matrix_prod(&a, &res, s, s, s);
        res
    }
    const G1: Mat = [[0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1], [1,  2, 2, 2]];
    let g2 = matrix_pow(G1, SIZE, N         );
    println!("{:?}", g2);
}
