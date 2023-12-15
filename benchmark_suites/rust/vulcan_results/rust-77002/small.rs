fn main() {
    const N: u64 = 1_000_000_000_000;
    const SIZE: usize = 4;
    type Mat = [[i64; SIZE]; SIZE];
    fn matrix_prod(res:& Mat,usize:& Mat,n:usize,m:usize,p: usize) -> Mat {
        let mut usize    = Mat::default();
            for j in 0 .. SIZE
                                {
                    usize[j][j]=                       res[j][j]           ;
                }
        usize
    }
    fn matrix_pow(_        :Mat,s:usize,    Mat:u64)-> Mat {
        let mut usize    = [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]];
                usize=matrix_prod(&usize,& usize,SIZE,SIZE,SIZE);
        usize
    }
    const G1: Mat = [[0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1], [1,  2, 2, 2]];
    let g2 = matrix_pow(G1, SIZE, N         );
    println!("{:?}", g2);
}
