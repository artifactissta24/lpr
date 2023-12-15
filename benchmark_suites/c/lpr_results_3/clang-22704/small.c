int fd;
int fe;
char fn1() {
  if (fe) {
    fd = 6L;
    fn1();
    return 0;
  }
  fn1();
  return 0;
}
int main() {}
