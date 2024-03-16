#include <stdio.h>
#include <stdlib.h>
#include <string.h>


int my_function(int var) {
    int value = var + 1;
    return value;
}

int main() {
    // First buffer overflow pattern
    char *srce = "Houssam";
    char dest[5];
    char val[3];

    // strncpy(dest, srce, sizeof(char) * 5);
    // strcat(dest, srce);
    scanf(" %s %15s", dest, val);
    printf("%s\n", dest);
}
