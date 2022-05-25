#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

pthread_rwlock_t rwlock = PTHREAD_RWLOCK_INITIALIZER;

void* reader(void *arg) {
    if (pthread_rwlock_rdlock(&rwlock) != 0) {
        perror("pthread_rwlock_rdlock");
        exit(-1);
    }

    if (pthread_rwlock_unlock(&rwlock) != 0) {
        perror("pthread_rwlock_unlock");
        exit(-1);
    }

    return NULL;
}

void* writer(void *arg) {
    if (pthread_rwlock_wrlock(&rwlock) != 0) {
        perror("pthread_rwlock_wrlock");
        exit(-1);
    }

    if (pthread_rwlock_unlock(&rwlock) != 0) {
        perror("pthread_rwlock_unlock");
        exit(-1);
    }

    return NULL;
}

int main(int argc, char *argv[]) {
    pthread_t r, w;

    if (pthread_create(&r, NULL, reader, NULL) != 0) {
        perror("pthread_create");
        exit(-1);
    }

    if (pthread_create(&w, NULL, writer, NULL) != 0) {
        perror("pthread_create");
        exit(-1);
    }

    if (pthread_join(r, NULL) != 0) {
        perror("pthread_join");
        exit(-1);
    }

    if (pthread_join(w, NULL) != 0) {
        perror("pthread_join");
        exit(-1);
    }

    if (pthread_rwlock_destroy(&rwlock) != 0) {
        perror("pthread_rwlock_destroy");
        exit(-1);
    }

    return 0;
}