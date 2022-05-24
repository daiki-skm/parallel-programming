#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

void *thread_func(void *arg) {
    for (int i = 0; i < 5; ++i) {
        printf("i = %d\n", i);
        sleep(1);
    }

    return NULL;
}

int main(int args, char *argv[]) {
    pthread_attr_t attr;
    if (pthread_attr_init(&attr) != 0) {
        perror("pthread_attr_init");
        return -1;
    }

    if (pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED) != 0) {
        perror("pthread_attr_setdetachstate");
        return -1;
    }

    pthread_t th;
    if (pthread_create(&th, &attr, thread_func, NULL) != 0) {
        perror("pthread_create");
        return -1;
    }

    if (pthread_attr_destroy(&attr) != 0) {
        perror("pthread_attr_destroy");
        return -1;
    }

    sleep(7);

    return 0;
}