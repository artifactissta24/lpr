char fn2(p1) { return p1; }
unsigned v_value;
unsigned u;
int main() {
  int d = 2;
  for (; u < 51; ++u)
    d = fn2(v_value = d) + 0x9EE7F9D1C772505BLL;
  printf("checksum = %x\n", v_value);
  return 0;
}
