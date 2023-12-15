struct original_data_type {
  int i;
  long j;
  unsigned k;
};
static unsigned fn1(unsigned p1, struct original_data_type p5) {
  p5.k;
w:
  goto w;
}
unsigned fn2() {
  struct original_data_type p;
  fn1(0, p);
  while (1) {
    struct original_data_type r;
    p = r = p;
  }
}
int main() {}
