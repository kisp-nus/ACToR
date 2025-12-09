CC = gcc
CFLAGS = -Wall -Wextra -O2 -std=c99 -D_GNU_SOURCE -DSUPPORT_UTMP
CFLAGS_REF = -Wall -Wextra -g -O0 -fprofile-arcs -ftest-coverage -std=c99 -D_GNU_SOURCE -DSUPPORT_UTMP
LIBS = 
TARGET = who
TARGET_REF = who.ref
SOURCES = who.c utmpentry.c util.c

$(TARGET): $(SOURCES)
	$(CC) $(CFLAGS) -o $(TARGET) $(SOURCES) $(LIBS)

$(TARGET_REF): $(SOURCES)
	$(CC) $(CFLAGS_REF) -o $(TARGET_REF) $(SOURCES) $(LIBS)

all: $(TARGET) $(TARGET_REF)

clean:
	rm -f $(TARGET) $(TARGET_REF) *.o *.gcda *.gcno *.gcov

.PHONY: clean all