#include <stdio.h>
#include <stdlib.h>
#include <string.h>


int main() {
    // First buffer overflow pattern
    char *srce = "Houssam";
    char dest[5];

    strcpy(dest, srce);
    printf("%s\n", dest);

    
}
