struct h {
  int i;
  long j;
  int k;
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
    struct h r;
    p = r = p;
  }
}
int main() {}
