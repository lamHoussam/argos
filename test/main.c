#include <stdio.h>
#include <string.h>

int main() {
    char buf[10];
    // strcpy(buf, "Really long text!");
    scanf("%15s", buf);
    printf("%s\n", buf);
    return 0;
}

