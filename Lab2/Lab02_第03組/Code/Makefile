# CROSS_COMPILE = aarch64-linux-gnu-
CROSS_COMPILE = ~/l4t-gcc/gcc-linaro-7.3.1-2018.05-x86_64_aarch64-linux-gnu/bin/aarch64-linux-gnu-

CXX = $(CROSS_COMPILE)g++ 
CXXFLAGS=-std=c++11
LD = $(CROSS_COMPILE)ld

SRC=$(wildcard *.cpp)
OBJ=$(SRC:.cpp=.o)
TARGET=Program

.PHONY: all clean upload run

all:$(TARGET)

$(TARGET):$(OBJ)
	$(CXX) -o $@ $^

%.o:%.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

clean:
	rm -f $(OBJ) $(TARGET)

upload: $(TARGET)
	scp $(TARGET) nvidia@192.168.55.1:/tmp/

run: upload
	ssh nvidia@192.168.55.1 '/tmp/$(TARGET)'
