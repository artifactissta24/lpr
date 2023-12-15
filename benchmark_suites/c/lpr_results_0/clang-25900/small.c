unsigned fn1(p2) { return p2; }
struct m {
  signed n : 18;
  signed o : 1;
};
int r;
unsigned t;
int main() {
  struct m ad = {1957};
  ad.o = fn1(5);
  int ae = ad.o;
  ad.o = 0;
  r = t % (4 / ae);
  ad.o;
}
