int f;
int fn1() {
  if (f) {
    fn1();
    return 0;
  }
  f = 6;
  fn1();
  return 0;
}
int main() {}
