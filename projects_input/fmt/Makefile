CC = gcc
CFLAGS = -Wall -Wextra -O2 -std=c99
CFLAGS_REF = -Wall -Wextra -g -O0 -fprofile-arcs -ftest-coverage -std=c99
LIBS = 
TARGET = fmt
TARGET_REF = fmt.ref
SOURCES = fmt.c

$(TARGET): $(SOURCES)
	$(CC) $(CFLAGS) -o $(TARGET) $(SOURCES) $(LIBS)

$(TARGET_REF): $(SOURCES)
	$(CC) $(CFLAGS_REF) -o $(TARGET_REF) $(SOURCES) $(LIBS)

all: $(TARGET) $(TARGET_REF)

clean:
	rm -f $(TARGET) $(TARGET_REF) *.gcda *.gcno *.gcov *.profdata *.profraw

.PHONY: clean all