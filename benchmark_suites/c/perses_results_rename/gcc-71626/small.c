typedef long a;
fn1() {}
typedef a d __attribute__((__vector_size__(sizeof(a))));
d fn2() {
  a c = fn1;
  d f = {c};
  return f;
}
main() {}
