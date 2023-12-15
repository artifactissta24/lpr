#include <stdio.h>
int v;
unsigned w = 0xDC6F3D13L;
long long ag_3;
int ai;
int ak;
int ao;
unsigned ap;
int fn2(int p1, int p2) { return p1 && p2 ? p1 : p1 % p2; }
int fn3(int p1, int p2) { return p2; }
int fn5(int p1, int p2) { return p2; }
int fn7(int p1, int p2) { return p1; }
long long fn8(int p1, long long p2) { return p1 + p2; }
int main() {
  int au_0;
  for (ak = 0;;) {
    for (ai = 0; ai <= 3; ai += 1)
      if (w >=
          (fn7(4UL,
               fn7(4UL, fn5(ai, au_0 = fn8(~(fn3(0, fn2(ap, 1L)) != 0), ak) &&
                                       0x57L))) > 2)) {
        v = 0x41AEA787L;
        ag_3 = au_0;
      } else
        ao = au_0;
    printf("checksum = %d\n", v);
    return 0;
  }
}
