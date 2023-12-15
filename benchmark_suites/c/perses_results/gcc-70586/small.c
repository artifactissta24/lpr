typedef signed int8_t;
typedef short int16_t;
typedef int int32_t;
typedef unsigned uint8_t;
typedef unsigned uint16_t;
typedef unsigned uint32_t;
int8_t safe_add_func_int8_t_s_s(int8_t si1, int8_t si2) { return si1 + si2; }
int8_t safe_lshift_func_int8_t_s_u(int8_t left, unsigned right) { return left; }
int32_t safe_mod_func_int32_t_s_s(int32_t si1, int32_t si2) {
  return si2 == 0 || si1 && si2 ? si1 : si1 % si2;
}
uint16_t safe_rshift_func_uint16_t_u_s(uint16_t left, int right) {}
uint32_t safe_add_func_uint32_t_u_u(uint32_t ui1, uint32_t ui2) {
  return ui1 + ui2;
}
uint32_t safe_sub_func_uint32_t_u_u(uint32_t ui1, uint32_t ui2) {
  return ui1 - ui2;
}
int8_t g_4;
int8_t g_83;
int16_t g_188;
static uint16_t g_538 = 0x2A52L;
uint32_t g_540 = 0x03F238A4L;
uint16_t g_813[7][1];
int32_t g_1215[6];
static uint16_t func_8(int16_t);
static int32_t func_1() {
  if (func_8(1))
    ;
}
uint16_t func_8(int16_t p_9) {
  int32_t l_26;
  l_26 = safe_mod_func_int32_t_s_s(p_9, g_4);
  if (g_540) {
    int32_t l_1179;
    int16_t l_1271 = 0x809CL;
    uint32_t l_1288;
    for (; p_9; --p_9)
      ;
    for (l_1288 = 0; l_1288 <= 49;
         l_1288 = safe_add_func_uint32_t_u_u(l_1288, 8)) {
      int32_t l_1386[8][6][5] = {1L};
      uint32_t l_1423;
      if (safe_lshift_func_int8_t_s_u(
              p_9, safe_add_func_int8_t_s_s(l_1386[6][5][0], p_9) ||
                       p_9 && safe_rshift_func_uint16_t_u_s(
                                  l_1179 = safe_mod_func_int32_t_s_s(
                                      p_9, 0xBC2AB98CL),
                                  9)))
        ;
      else {
        uint16_t BGgGgfl_1401 = l_26;
        l_1423 = BGgGgfl_1401;
        if (!g_538) {
          printf("l_1179=%lld\n", (long long)l_1179);
          break;
        }
      }
      p_9 = safe_sub_func_uint32_t_u_u(
          0x60781BCDL >= safe_mod_func_int32_t_s_s(
                             safe_mod_func_int32_t_s_s(l_1271, p_9) > 0xB5L,
                             p_9) !=
                  p_9 ^
              4294967287UL,
          l_1423);
      g_83 = p_9;
      l_26 = safe_mod_func_int32_t_s_s(0x7D765DB6L, g_1215[4]);
    }
  } else {
    uint8_t l_1955[3];
    g_813[2][0] = safe_mod_func_int32_t_s_s(
        safe_mod_func_int32_t_s_s(l_1955[1], g_188), 0x9CFB623FL);
  }
  return p_9;
}
int main() { func_1(); }
