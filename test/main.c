#include <stdio.h>
#include <stdlib.h>
#include <string.h>
// #include <time.h>
// #include <unistd.h>
// #include<sys/ipc.h>
// #include<sys/shm.h>
// #include<sys/types.h>
// #include<errno.h>

// extern void malloc_intercept(int size);
// extern void free_intercept();


int main(int argc, char *argv[]) {
    // struct MyStruct s = {.value = 5};

    // int shmid, numtimes;
    // struct shmseg *shmp;
    // char *bufptr;
    // int spaceavailable;
    // shmid = shmget(SHM_KEY, sizeof(struct MyStruct), 0666|IPC_CREAT);
    // if (shmid == -1) {
    //     perror("Shared memory");
    //     return 1;
    // }

    // // Attach to the segment to get a pointer to it.
    // shmp = shmat(shmid, NULL, 0);
    // if (shmp == (void *) -1) {
    //     perror("Shared memory attach");
    //     return 1;
    // }

    // printf("Created Shared mem %p\n", shmp);

    

    char* values = malloc(sizeof(char) * 6);

    // malloc_intercept(5);
    // free_intercept();

    // sleep(2);
    strcpy((char*)values, "Helloooooooooooo");

    free(values);
    // char buf[10];
    // strcpy(buf, "Really long text!");
    // scanf("%15s", buf);

    // strcpy(buf, argv[1]);
    // printf("%s\n", buf);
    return 0;
}

