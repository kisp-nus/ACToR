CC = gcc
CFLAGS = -Wall -Wextra -O2 -std=c99 -D_GNU_SOURCE
CFLAGS_REF = -Wall -Wextra -g -O0 -fprofile-arcs -ftest-coverage -std=c99 -D_GNU_SOURCE
LIBS = 
TARGET = tr
TARGET_REF = tr.ref
SOURCES = tr.c str.c

$(TARGET): $(SOURCES)
	$(CC) $(CFLAGS) -o $(TARGET) $(SOURCES) $(LIBS)

$(TARGET_REF): $(SOURCES)
	$(CC) $(CFLAGS_REF) -o $(TARGET_REF) $(SOURCES) $(LIBS)

all: $(TARGET) $(TARGET_REF)

clean:
	rm -f $(TARGET) $(TARGET_REF) *.gcno *.gcda *.gcov

.PHONY: clean all