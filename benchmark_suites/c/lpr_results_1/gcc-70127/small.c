unsigned u;
typedef struct {
  int ab;
  signed q : 22;
} p;
p v[1];
p w = {0x12345678, 999};
int main() {
  p a = {8L, 666};
  for (; u <= 1; u += 1) {
    a = v[0] = w;
    v[0] = a;
  }
  printf("checksum = %X\n", v[0].q);
  return 0;
}
