CC = gcc
CFLAGS = -I./include -Wall -Wextra -O2 -g
CFLAGS_REF = -I./include -Wall -Wextra -g -O0 -fprofile-arcs -ftest-coverage

SRCS := $(wildcard *.c)
OBJS := $(SRCS:.c=.o)
OBJS_REF := $(SRCS:.c=.ref.o)

TARGET = tail
TARGET_REF = tail.ref

.PHONY: all
all: $(TARGET) $(TARGET_REF)

$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) $^ -o $@

$(TARGET_REF): $(OBJS_REF)
	$(CC) $(CFLAGS_REF) $^ -o $@ -lgcov

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

%.ref.o: %.c
	$(CC) $(CFLAGS_REF) -c $< -o $@

.PHONY: clean
clean:
	rm -rf $(OBJS) $(OBJS_REF) $(TARGET) $(TARGET_REF) *.gcno *.gcda *.gcov
