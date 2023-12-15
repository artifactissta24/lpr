int fn3(int p1, int p2) {
  return p1 && p2 && p1 > 2147483647 - p2 || p1 < 0 && p2 < 0 && p1 ? p1
                                                                    : p1 + p2;
}
unsigned n;
signed o;
int main() {
  for (; n < 12; n = fn3(n, 3)) {
    int ab = o == 0 ? o : o % o;
    if (o <= 0xE3L)
      ;
    else if (ab)
      break;
  }
  return 0;
}
