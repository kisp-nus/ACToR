CC = gcc
CFLAGS = -Wall -Wextra -O2 -std=c99 -D_GNU_SOURCE
CFLAGS_REF = -Wall -Wextra -g -O0 -fprofile-arcs -ftest-coverage -std=c99 -D_GNU_SOURCE
TARGET = tsort
TARGET_REF = tsort.ref
SOURCES = tsort.c util.c

$(TARGET): $(SOURCES)
	$(CC) $(CFLAGS) -o $(TARGET) $(SOURCES)

$(TARGET_REF): $(SOURCES)
	$(CC) $(CFLAGS_REF) -o $(TARGET_REF) $(SOURCES)

all: $(TARGET) $(TARGET_REF)

clean:
	rm -f $(TARGET) $(TARGET_REF) *.gcno *.gcda *.gcov

.PHONY: clean all