CC = gcc
CFLAGS = -Wall -Wextra -O2 -std=c99
CFLAGS_REF = -Wall -Wextra -g -O0 -fprofile-arcs -ftest-coverage -std=c99
TARGET = printf
TARGET_REF = printf.ref
SOURCES = printf.c

$(TARGET): $(SOURCES)
	$(CC) $(CFLAGS) -o $(TARGET) $(SOURCES)

$(TARGET_REF): $(SOURCES)
	$(CC) $(CFLAGS_REF) -o $(TARGET_REF) $(SOURCES)

all: $(TARGET) $(TARGET_REF)

clean:
	rm -f $(TARGET) $(TARGET_REF) *.gcda *.gcno *.gcov *.profdata *.profraw

.PHONY: clean all