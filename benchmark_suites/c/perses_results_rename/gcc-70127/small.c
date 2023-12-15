typedef signed a;
typedef int c;
typedef unsigned d;
typedef unsigned e;
typedef unsigned f;
typedef unsigned g;
void fn1(f p1, int p2) {
  printf("checksum = %X\n", p1);
}
f j;
void fn2(p1) { j = p1; }
void fn3(p1) { fn2(p1); }
void fn4(g p1, char *p2, int p3) { fn3(p1); }
struct p {
  c ab;
  signed q : 22;
};
struct r {
  unsigned ab;
  unsigned s;
};
struct r t;
f u;
struct p v[1];
struct p w = {0x1C87E74FL, 816};
d fn5(e, a, d, f, f);
a fn6() { fn5(t.ab, 3, 2, t.s, 9); }
d fn5(e p1, a p2, d p3, f p4,
                f p5) {
  struct p a = {8L, 666};
  for (p2 = 0; p2 >= 0; p2 -= 1)
    for (; u <= 1; u += 1) {
      a = v[p2] = w;
      v[p2] = a;
    }
}
int main() {
  int i;
  int b = 0;
  fn6();
  for (i = 0; i < 1; i++)
    fn4(v[i].q, "g_217[i].f1", b);
  fn1(j, b);
  return 0;
}
