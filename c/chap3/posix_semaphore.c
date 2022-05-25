#include <pthread.h> // c1
#include <fcntl.h>
#include <sys/stat.h>
#include <semaphore.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#define NUM_THREADS 10
#define NUM_LOOP 10

int count = 0; // c2

void *th(void *arg) {
    // c3
    sem_t *s = sem_open("/mysemaphore", 0);
    if (s == SEM_FAILED) {
        perror("sem_open");
        exit(1);
    }

    for (int i = 0; i < NUM_LOOP; i++) {
        // c4
        if (sem_wait(s) == -1) {
            perror("sem_wait");
            exit(1);
        }

        __sync_fetch_and_add(&count, 1);
        printf("count = %d\n", count);

        usleep(10000);

        __sync_fetch_and_sub(&count, 1);

        // c5
        if (sem_post(s) == -1) {
            perror("sem_post");
            exit(1);
        }
    }

    // c6
    if (sem_close(s) == -1) {
        perror("sem_close");
    }

    return NULL;
}

int main(int argc, char *argv[]) {
    // c7
    sem_t *s = sem_open("/mysemaphore", O_CREAT, 0660, 3);
    if (s == SEM_FAILED) {
        perror("sem_open");
        return 1;
    }

    pthread_t v[NUM_THREADS];
    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_create(&v[i], NULL, th, NULL);
    }

    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_join(v[i], NULL);
    }

    if (sem_close(s) == -1) {
        perror("sem_close");
    }

    // c8
    if (sem_unlink("/mysemaphore") == -1) {
        perror("sem_unlink");
    }

    return 0;
}
