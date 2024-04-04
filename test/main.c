#include <stdio.h>
#include <string.h>

int main(int argc, char *argv[]) {
    char buf[10];
    // strcpy(buf, "Really long text!");
    // scanf("%15s", buf);

    strcpy(buf, argv[1]);
    printf("%s\n", buf);
    return 0;
}

