int fn4(int p1, int p2) { return p2 % p1; }
unsigned u;
int v;
static unsigned w = 254U;
signed ad_dim1[][5][7] = {{8}, {8}, {{8}, 0x98}};
int af;
signed fn2(signed p1, signed p2) { return p1; }
signed optimized_fn9() {
  int ai[2];
  int j;
  for (j = 0; j < 2; j++)
    ai[j] = 8;
  for (w = 1;;) {
    for (; j < 4; j++) {
    }
    for (af = 6; af >= 0; af -= 1)
      v = ai[0] ^= ad_dim1[w + 1][w][af] = fn4(1, fn2(3, u--));
    return 0;
  }
}
int main() {
  optimized_fn9();
  printf("checksum = %x\n", ad_dim1[2][1][0]);
  return 0;
}
