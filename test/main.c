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

    // strncpy(dest, srce, sizeof(char) * 5);
    strcpy(dest, srce);
    printf("%s\n", dest);    
}
