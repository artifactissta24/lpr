char fn1(unsigned p1, int p2) { return p1 > p2 ? p1 : p1 << p2; }
struct i {
  int j;
  int k;
  int l;
};
unsigned m[4];
struct i n[2];
struct i fn2(int p1, struct i p2, unsigned p3, short p4) {
  int v_j = 0x15L;
  int v_l;
  int w;
  int a;
  if (p4 ^ -5L)
    ;
  else
    v_j = n[1].j;
  for (; w; w += 3)
    for (p3 = 0; p3 <= 39; ++p3)
      for (p2.j = 0; p2.j <= 2; p2.j += 1) {
        n[0].j = v_j;
        n[0].l = v_l;
        v_l = m[p2.j];
        if (!n[0].j)
          ;
        else {
          w = fn1(n[0].l, 2) && p2.k;
          for (; a; a += 1)
            n[0] = (struct i){v_j};
        }
      }
  return p2;
}
unsigned fn3(p1) {
  unsigned char q[7][7];
  struct i r;
  if (fn2(p1, r, p1, q[0][0]), p1)
    ;
  return p1;
}
int main() {}
