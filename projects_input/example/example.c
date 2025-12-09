#include <stdio.h>
#include <string.h>
#include <stdlib.h>

// Simple foo functionality: reverses the given string
void foo(const char *input) {
    size_t len = strlen(input);
    char *reversed = (char *)malloc(len + 1);
    if (!reversed) {
        fprintf(stderr, "Memory allocation failed\n");
        return;
    }

    for (size_t i = 0; i < len; i++) {
        reversed[i] = input[len - 1 - i];
    }
    reversed[len] = '\0';

    printf("Foo (reverse): %s\n", reversed);

    free(reversed);
}

int main(int argc, char *argv[]) {
    if (argc < 2) {
        printf("Usage: %s <string>\n", argv[0]);
        return 1;
    }
    foo(argv[1]);
    return 0;
}
