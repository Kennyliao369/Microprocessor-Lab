CROSS_COMPILE = aarch64-linux-gnu-
# CROSS_COMPILE = ~/l4t-gcc/gcc-linaro-7.3.1-2018.05-x86_64_aarch64-linux-gnu/bin/aarch64-linux-gnu-

# 指定編譯套件
CXX = $(CROSS_COMPILE)g++ 
# 指定編譯參數
CXXFLAGS=-std=c++11
LD = $(CROSS_COMPILE)ld

# 通配符匹配所有當前資料夾的 .cpp 的檔案
SRC=$(wildcard *.cpp)
# 替換模式 將 SRC 的 .cpp 換成 .o 
OBJ=$(SRC:.cpp=.o)
# 輸出檔案名稱
EXE=Area

# 宣告 make 參數（防出錯）
.PHONY: all clean

# 預設入口
# 冒號左邊是輸出，右邊是要用到的材料
all:$(EXE)

# 表示產出 EXE 前要用到 OBJ
$(EXE):$(OBJ)
	# 連接階段：$@ 表示冒號右邊的內容，$^ 表示冒號左邊的所有內容
	$(CXX) -static -o $@ $^

# 匹配模式。一次匹配一組同名的 .o 與 .cpp 。
%.o:%.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

# 清理編譯產物
clean:
	rm -f $(OBJ)
	rm -f $(EXE)

# 項目一: Makefile要有target: all (20%)
# • 編譯出跨平台可執行檔
# • 項目二: Makefile要有target: clean (20%)
# • 刪除該執行檔與所有.o檔
# • 項目三: 在 Makefile 中使用變數，可參考16頁 (10%)
# • 項目四: 在 Makefile 裡須加入 $@ (5%) ， $< or $? (5%)
# • 項目五: 將虛擬機CROSS_COMPILE後的執行檔傳至TX2，且以ssh 連到TX2執行 (10%)
# • 項目六: 實驗報告與Makefile檔 (30%)
