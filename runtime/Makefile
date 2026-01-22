CC = gcc
CFLAGS = -Wall -Wextra -std=c99 -O2 -fPIC -Iinclude
LDFLAGS = -shared

SRC_DIR = .
MEM_DIR = mem
STR_DIR = str
INCLUDE_DIR = include
BUILD_DIR = build
LIB_DIR = lib

STATIC_LIB = libruntime.a

MEM_SRCS = $(MEM_DIR)/alloc_node.c $(MEM_DIR)/gc.c $(MEM_DIR)/vec.c
STR_SRCS = $(STR_DIR)/str.c
SRCS = $(MEM_SRCS) $(STR_SRCS)

MEM_OBJS = $(patsubst $(MEM_DIR)/%.c,$(BUILD_DIR)/mem/%.o,$(MEM_SRCS))
STR_OBJS = $(patsubst $(STR_DIR)/%.c,$(BUILD_DIR)/str/%.o,$(STR_SRCS))
OBJS = $(MEM_OBJS) $(STR_OBJS)

all: directories $(LIB_DIR)/$(LIB_NAME) $(LIB_DIR)/$(STATIC_LIB)

directories:
	@mkdir -p $(BUILD_DIR)/mem
	@mkdir -p $(BUILD_DIR)/str
	@mkdir -p $(LIB_DIR)

$(LIB_DIR)/$(STATIC_LIB): $(OBJS)
	ar rcs $@ $^
	@echo "Built static library: $@"

$(BUILD_DIR)/mem/%.o: $(MEM_DIR)/%.c
	$(CC) $(CFLAGS) -c $< -o $@

$(BUILD_DIR)/str/%.o: $(STR_DIR)/%.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -rf $(BUILD_DIR) $(LIB_DIR)

rebuild: clean all

debug: CFLAGS += -g -DDEBUG -O0
debug: clean all

info:
	@echo "CC: $(CC)"
	@echo "CFLAGS: $(CFLAGS)"
	@echo "SRCS: $(SRCS)"
	@echo "OBJS: $(OBJS)"

.PHONY: all directories clean rebuild install uninstall debug info
