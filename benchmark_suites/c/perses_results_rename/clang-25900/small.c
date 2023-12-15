typedef signed a;
typedef short b;
typedef int c;
typedef long d;
typedef unsigned e;
typedef unsigned f;
typedef unsigned g;
e fn1(e p1, int p2) { return p2; }
struct k {
  f l;
};
struct m {
  signed n : 18;
  signed o : 1;
};
a p;
struct k q;
c r;
static a s = -5L;
g t;
c u;
g v;
a fn2(a, struct m, struct k, struct k, c);
b fn3(g);
d fn4() {
  struct m z;
  fn2(p, z, q, q, q.l);
}
a fn2(a p1, struct m p2, struct k p3, struct k p4,
               c p5) {
  fn3(p5);
}
b fn3(g p1) {
  struct m ad = {1957};
  ad.o = fn1(v, 5);
  a ae;
  c af = t;
  ae = ad.o;
  ad.o = u;
  r = af % (~s / ae);
  ad.o;
}
int main() { fn4(); }
