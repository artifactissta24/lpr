typedef char a;
typedef short c;
typedef int d;
typedef unsigned e;
typedef unsigned f;
typedef unsigned g;
void fn1(f p1, int p2) {
  printf("checksum = %X\n", p1);
}
a fn2(a p1, int p2) { return p1; }
c fn3(c p1, c p2) { return p2; }
g fn4(g p1, g p2) {
  return p1 + p2;
}
f p;
void fn5(p1) { p = p1; }
void fn6(p1) { fn5(p1); }
void fn7(g p1, char *p2, int p3) { fn6(p1); }
f t;
g u;
g v[1][4];
e fn8(a, d);
g fn9(f, f, d, g, d);
d fn10(d, e, f, d);
c fn11() {
  a a = 0xFBL;
  d b = 0L;
  fn8(t, a);
  return b;
}
e fn8(a p1, d p2) {
  fn9(p2, p2, t, 18446744073709551613UL, t);
  return p1;
}
g fn9(f p1, f p2, d p3, g p4,
                 d p5) {
  e c = 255UL;
  fn10(6, c, 1, p1);
  return p2;
}
d fn10(d p1, e p2, f p3, d p4) {
  d d = 2L;
  for (; u < 51; ++u) {
    g e[9];
    int i;
    for (i = 0; i < 9; i++)
      e[i] = 0x9EE7F9D1C772505BLL;
    d = fn4(
        fn2(
            v[0][3] = fn3(4L, d), 3),
        e[0]);
  }
  return 3;
}
int main() {
  int i, j;
  int f = 0;
  fn11();
  for (i = 0; i < 1; i++)
    for (j = 0; j < 4; j++)
      fn7(v[i][j], "g_1729[i][j]", f);
  fn1(p, f);
  return 0;
}
