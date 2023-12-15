int fn4(int p1, int p2) { return p1 % p2; }
int fn5(int p1, int p2) { return p1; }
int s;
int u;
int v;
static int w = 254;
int ad[][5][7] = {{8}, {8}, {{8}, 0x98}};
int af;
int fn9() {
  int ai[2];
  int i;
  for (i = 0; i < 2; i++)
    ai[i] = 8;
  for (w = 1;;) {
    for (; i < 2; i++)
      ;
    for (af = 6; af >= 0; af -= 1)
      v = ai[0] ^= ad[w + 1][w][af] = fn4(fn5(1, fn5(255, u--)), 4294967295);
    return af;
  }
}
int main() {
  fn9();
  s = ad[2][1][0];
  printf("checksum = %d\n", s);
  return 0;
}
