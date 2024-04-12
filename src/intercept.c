#define _GNU_SOURCE
#include <dlfcn.h>
#include <stdio.h>

// extern void malloc_intercept(char* var_name, size_t size, char* var_type);
extern void malloc_intercept(int size, void* ptr);
extern void free_intercept(void* ptr);

void* malloc(size_t size) {
    static void* (*real_malloc)(size_t) = NULL;
    if (!real_malloc) {
        real_malloc = dlsym(RTLD_NEXT, "malloc");
    }

    void* p = real_malloc(size);
    malloc_intercept((int)size, p);
    return p;
}


void free(void* ptr) {
    static void* (*real_free)(void*) = NULL; 
    if (!real_free) {
        real_free = dlsym(RTLD_NEXT,"free");
    }

    // printf("Free called\n");
    free_intercept(ptr);
    real_free(ptr);
}

