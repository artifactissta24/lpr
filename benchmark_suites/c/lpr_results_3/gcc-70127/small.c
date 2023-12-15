#include <stdio.h>

unsigned u;

signed ab;
signed q;

struct p {
  signed ab;
  signed q : 22;
};

struct p v[1];
struct p w_init = {0x1C87E74FL, 816};

int main() {
  struct p a = {8, 666};

  for (; u <= 1; u += 1) {
    a = v[0] = w_init;
    v[0] = a;
  }

  printf("checksum = %X\n", v[0].q);

  return 0;
}
