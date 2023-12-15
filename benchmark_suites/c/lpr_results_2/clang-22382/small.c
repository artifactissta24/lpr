struct h {
  int i;
  long j;
  unsigned k;
};
static struct h fn1(struct h p5) {
  p5.k;
w:
  goto w;
}
unsigned fn2() {
  struct h p;
  fn1(p);
  for (;;) {
    struct h r1;
    p = r1 = p;
  }
}
int main() {}
