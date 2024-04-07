#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>


extern void malloc_intercept(int size);
extern void free_intercept();


int main(int argc, char *argv[]) {
    // int* values = malloc(sizeof(int) * 5);

    malloc_intercept(5);
    free_intercept();

    sleep(2);

    // free(values);
    // char buf[10];
    // strcpy(buf, "Really long text!");
    // scanf("%15s", buf);

    // strcpy(buf, argv[1]);
    // printf("%s\n", buf);
    return 0;
}

