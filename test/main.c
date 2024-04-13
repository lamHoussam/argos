#include <stdio.h>
#include <stdlib.h>
#include <string.h>


void static_parer_test() {
    char buf[10];
    strcpy(buf, "Really long text!");
    printf("%s\n", buf);
}

void dynamic_parser_test() {
    char* values = malloc(sizeof(char) * 6);
    strcpy(values, "Hello");
    printf("%s\n", values);
    free(values);
}

void data_leak_check_test(int num) {
    for (int i = 0; i < num; i++) {
        char* values = malloc(sizeof(char) * 6);
        printf("%s\n", values);
    }
}

int main(int argc, char *argv[]) {
    static_parer_test();
    dynamic_parser_test();
    data_leak_check_test(5);

    return 0;
}

