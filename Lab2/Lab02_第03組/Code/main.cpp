#include <chrono>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <thread>

#include "gpio.hpp"

// pin: 7 13 29 33
const int LED_COUNT = 4;
const int LEDS[LED_COUNT] = {396, 255, 398, 389};

// tool
const char *trim_first(const char *src, const char *s) {
  size_t n = strlen(s);
  return strncmp(src, s, n) == 0 ? src + n : src;
}

int run_led(int pin, int state) {
  // 掛載
  gpio::onexport(pin);
  gpio::set_direction(pin, "out");

  // 執行
  gpio::set_value(pin, state);

  // 釋放
  if (state == 0) {
    gpio::unexport(pin);
  }
  return 0;
}

int run_shine(int count) {
  // 掛載
  for (int i = 0; i < LED_COUNT; i++) {
    gpio::onexport(LEDS[i]);
    gpio::set_direction(LEDS[i], "out");
  }

  // 執行
  for (int i = 0; i < count; i++) {
    for (int index = 0; index < LED_COUNT; index++) {
      int offset = index < 2 ? 1 : 0;
      gpio::set_value(LEDS[index], (i + offset) % 2);
    }
    std::this_thread::sleep_for(std::chrono::milliseconds(500));
  }

  // 釋放
  for (int i = 0; i < LED_COUNT; i++) {
    gpio::unexport(LEDS[i]);
  }
  return 0;
}

int main(int argc, char *argv[]) {
  if (argc < 3) {
    fprintf(stderr, "Argument not enough\n");
    return 1;
  }

  // LED 模式
  if (strncmp("LED", argv[1], 3) == 0) {
    // 解析參數
    int index = atoi(trim_first(argv[1], "LED"));
    if (index < 1 || index > LED_COUNT) {
      fprintf(stderr, "LED index parse error\n");
      return 1;
    }
    int pin = LEDS[index - 1];

    int state;
    if (strcmp(argv[2], "on") == 0) {
      state = 1;
    } else if (strcmp(argv[2], "off") == 0) {
      state = 0;
    } else {
      fprintf(stderr, "LED state parse error\n");
      return 1;
    }

    // 執行
    run_led(pin, state);

    // Shine 模式
  } else if (strcmp("Shine", argv[1]) == 0) {
    // 解析參數
    int count = atoi(argv[2]);
    if (count <= 0) {
      fprintf(stderr, "Shine count parse error\n");
      return 1;
    }

    // 執行
    run_shine(count);

    // 例外
  } else {
    fprintf(stderr, "Unknown command\n");
    return 1;
  }

  return 0;
}

// /sys/class/gpio/
// echo 255 > export

// /sys/class/gpio/gpoi255
// echo out > direction
// echo 1 > value

// /sys/class/gpio/
// echo 255 > unexport
