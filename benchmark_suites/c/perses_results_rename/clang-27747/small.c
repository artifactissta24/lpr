typedef signed a;
typedef int b;
struct c {
  signed d : 18;
  unsigned e;
  unsigned f;
  signed g;
  signed h;
};
struct c i;
struct c j;
b fn1() {
  struct c l = {24, 6738, 165};
  b m = j.e;
  l.d = m;
  b n = j.h && l.d;
  b o = l.d;
  a p =
      o | -(l.f && l.d) - n;
  i.g = p;
}
int main() {}
