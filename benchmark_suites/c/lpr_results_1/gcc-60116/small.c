#include <stdio.h>
int fn2(int p1, int p2) { return p1 && p2 ? p1 : p1 % p2; }
int fn5(int p1, int p2) { return 1110486715; }
unsigned fn6(unsigned p1, unsigned p2) { return 0xDC6F3D13L; }
long long fn8(long p1, long long p2) { return p1 + p2; }
unsigned f;
unsigned w = 0xDC6F3D13L;
long long ag_3;
int ah;
int ak;
int am = 0x41AEA787L;
int ao;
unsigned ap;
int main() {
  int au_4_1_5;
  ak = 0;
  for (ah = 0; ah <= 3; ah += 1)
    if (w >= (fn6(0, fn5(1110486715, au_4_1_5 = fn8(~(fn2(ap, 1L) != 0), ak) &&
                                                0x57L)) > 2)) {
      f = am;
      ag_3 = au_4_1_5;
    } else
      ao = fn8(~(fn2(ap, 1L) != 0), ak) && 0x57L;
  printf("checksum = %X\n", f);
  return 0;
}
