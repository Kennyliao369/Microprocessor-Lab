#include "gpio.hpp"

#include <cstddef>
#include <cstdio>
#include <cstring>
#include <unistd.h>

namespace gpio {
int onexport(const int gpio) {
  const char *path = "/sys/class/gpio/export";

  // 判斷檔案已經存在
  char gpio_path[64];
  snprintf(gpio_path, sizeof(gpio_path), "/sys/class/gpio/gpio%d", gpio);
  if (access(gpio_path, F_OK) == 0) {
    return 0;
  }

  // 嘗試開檔
  FILE *file = fopen(path, "w");
  if (file == NULL) {
    perror("gpio export error");
    return 1;
  }

  // 寫入
  fprintf(file, "%d", gpio);

  // 關檔
  if (fclose(file) != 0) {
    perror("gpio export error");
    return 1;
  }
  return 0;
}

int unexport(const int gpio) {
  const char *path = "/sys/class/gpio/unexport";

  // 判斷檔案已經不存在
  char gpio_path[64];
  snprintf(gpio_path, sizeof(gpio_path), "/sys/class/gpio/gpio%d", gpio);
  if (access(gpio_path, F_OK)) {
    return 0;
  }

  // 嘗試開檔
  FILE *file = fopen(path, "w");
  if (file == NULL) {
    perror("gpio unexport error");
    return 1;
  }

  // 寫入
  fprintf(file, "%d", gpio);

  // 關檔
  if (fclose(file) != 0) {
    perror("gpio unexport error");
    return 1;
  }
  return 0;
}

int set_direction(const int gpio, const char *state) {
  // 檢查輸入
  if (strcmp("in", state) != 0 && strcmp("out", state) != 0) {
    fprintf(stderr, "Invalid gpio direction\n");
    return 1;
  }

  // 嘗試開檔
  char path[64];
  snprintf(path, sizeof(path), "/sys/class/gpio/gpio%d/direction", gpio);
  FILE *file = fopen(path, "w");
  if (file == NULL) {
    perror("gpio set direction error");
    return 1;
  }

  // 寫入
  fprintf(file, "%s", state);

  // 關檔
  if (fclose(file) != 0) {
    perror("gpio direction set error");
    return 1;
  }
  return 0;
}

int set_value(const int gpio, const int value) {
  // 檢查輸入
  if (value != 0 && value != 1) {
    fprintf(stderr, "Invalid gpio value\n");
    return 1;
  }

  // 嘗試開檔
  char path[64];
  snprintf(path, sizeof(path), "/sys/class/gpio/gpio%d/value", gpio);
  FILE *file = fopen(path, "w");
  if (file == NULL) {
    perror("gpio set value error");
    return 1;
  }

  // 寫入
  fprintf(file, "%d", value);

  // 關檔
  if (fclose(file) != 0) {
    perror("gpio value set error");
    return 1;
  }
  return 0;
}
} // namespace gpio
