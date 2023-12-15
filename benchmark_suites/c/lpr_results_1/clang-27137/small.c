short o;
int main() {
  int i;
  short r = o;
  for (i = 0; i < 10; i++)
    while (r > 1L)
      r -= 1;
  printf("checksum = %X\n", r);
  return 0;
}
