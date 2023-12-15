#include <stdint.h>
uint32_t m0[4];
struct i {
  int32_t j;
  int32_t l;
};
struct i n[2];
int main() {}
int fn1(uint8_t p1) { return p1 ? p1 : 0; }
int fn3() {
  uint32_t s;
  struct i v = {0x15L};
  int32_t w;
  int32_t a;
  for (; w; w += 1)
    for (s = 0; s <= 39; ++s)
      for (v.j = 0; v.j <= 2; v.j += 1) {
        n[0] = v;
        v.l = m0[v.j];
        w = fn1(n[0].l);
        for (; a; a -= 4)
          n[0] = v;
      }
  return 0;
}
