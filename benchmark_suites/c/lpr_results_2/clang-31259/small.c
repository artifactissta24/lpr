int fn4(int p1, int p2) { return p1 % p2; }
int u;
int v;
static int w = 254;
int ad[][5][7] = {{8}, {8}, {{8}, 0x98}};
int af;
int main() {
  int ai[2];
  int j;
  for (j = 0; j < 2; j++)
    ai[j] = 8;
  w = 1;
  for (; j < 25; j += 20) {
  }
  for (af = 6; af >= 0; af -= 1)
    v = ai[0] ^= ad[w + 1][w][af] = fn4(--u, 4294967295);
  printf("checksum = %X\n", ad[2][1][0]);
  return 0;
}
