int loop_var;
int main() {
  int q;
  q = loop_var;
  for (loop_var = 0; loop_var < 10; loop_var++)
    while (q > 1L)
      q -= 1;
  printf("checksum = %X\n", q);
  return 0;
}
