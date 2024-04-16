#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void dynamic_parser_test() {
    char* values = malloc(sizeof(char) * 6);
    strcpy(values, "H");
    printf("%s\n", values);
    free(values);
}

void data_leak_check_test(int num) {
    for (int i = 0; i < num; i++) {
        char* values = malloc(sizeof(char) * 6);
        printf("%s\n", values);
    }
}

// TODO: Manage scopes with a list of Hashmaps


void malloc_frenzy(int num) {
    for (int i = 0; i < num; i++) {
        char* values = malloc(sizeof(char) * 6);
        strcpy(values, "Really long text!");
        // printf("%s\n", values);
        // free(values);
    }
}

int main(int argc, char *argv[]) {
    // dynamic_parser_test();
    // data_leak_check_test(5);
    malloc_frenzy(80);

    return 0;
}

