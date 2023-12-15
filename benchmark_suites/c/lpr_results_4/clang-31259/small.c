int fn3(int p1, int p2) { return 1; }
int u;
int v;
static int w = 254;
int ad1[][5][7] = {{8}, {8}, {{8}, 0x98}};
int af;
int fn4(int p1, int p2) { return p1 % p2; }
int fn9() {
  int ai[2];
  int i;
  for (i = 0; i < 2; i++)
    ai[i] = 8;
  for (w = 1;;) {
    for (; i < 5; i += 2)
      ;
    for (af = 6; af >= 0; af -= 1)
      v = ai[0] ^= ad1[w + 1][w][af] = fn4(fn3(1, u--), 4294967295);
    return 0;
  }
}
int main() {
  fn9();
  int s;
  s = ad1[2][1][0];
  printf("s_unrolled_iter_1 = %X\n", s);
  return 0;
}
