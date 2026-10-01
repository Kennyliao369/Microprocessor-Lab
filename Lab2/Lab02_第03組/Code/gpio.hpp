#pragma once

namespace gpio {
int onexport(const int gpio);
int unexport(const int gpio);
int set_direction(const int gpio, const char *state);
int set_value(const int gpio, const int value);
} // namespace gpio
