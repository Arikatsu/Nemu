ASM        := rgbasm
LINK       := rgblink

SRC_DIR    := src
BUILD_DIR  := build

SRC        := $(SRC_DIR)/dmg_boot.asm
OBJ        := $(BUILD_DIR)/dmg_boot.o
BIN        := $(BUILD_DIR)/dmg_boot.bin

all: $(BIN)

$(BIN): $(OBJ)
	$(LINK) -x -o $@ $<

$(OBJ): $(SRC)
	mkdir -p $(BUILD_DIR)
	$(ASM) -o $@ $<

clean:
	rm -f $(BUILD_DIR)/*

.PHONY: all clean
