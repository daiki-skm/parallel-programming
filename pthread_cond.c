#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

pthread_mutex_t mut = PTHREAD_MUTEX_INITIALIZER; // c1
pthread_cond_t cond = PTHREAD_COND_INITIALIZER; // c2

volatile bool ready = false; // c3
char buf[256];

void* producer(void *arg) { // c4
    printf("producer: ");
    fgets(buf, sizeof(buf), stdin);

    pthread_mutex_lock(&mut);
    ready = true; // c5

    if (pthread_cond_broadcast(&cond) != 0) { // c6
        perror("pthread_cond_broadcast");
        exit(1);
    }

    pthread_mutex_unlock(&mut);
    return NULL;
}

void* consumer(void *arg) { // c7
    pthread_mutex_lock(&mut);

    while (!ready) {
        if (pthread_cond_wait(&cond, &mut) != 0) { // c8
            perror("pthread_cond_wait");
            exit(1);
        }
    }

    pthread_mutex_unlock(&mut);
    printf("consumer: %s\n", buf);
    return NULL;
}

int main(int argc, char *argv[]) {
    pthread_t prod, cons;
    pthread_create(&prod, NULL, producer, NULL);
    pthread_create(&cons, NULL, consumer, NULL);

    pthread_join(prod, NULL);
    pthread_join(cons, NULL);

    pthread_mutex_destroy(&mut);

    if (pthread_cond_destroy(&cond) != 0) { // c9
        perror("pthread_cond_destroy");
        return -1;
    }

    return 0;
}