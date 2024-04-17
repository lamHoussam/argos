#include <stdio.h>
#include <stdlib.h>
#include <string.h>


void static_parer_test() {
    char new_buf[16];
    char buf[8];
    strcpy(buf, "Really long text!");
    scanf("This: %19s, %s", new_buf, buf);
    printf("%s\n", new_buf);
}

// TODO: Manage scopes with a list of Hashmaps
int main(int argc, char *argv[]) {
    static_parer_test();

    return 0;
}