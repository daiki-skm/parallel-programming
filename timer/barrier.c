#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

pthread_mutex_t barrier_mut = PTHREAD_MUTEX_INITIALIZER;
pthread_cond_t barrier_cond = PTHREAD_COND_INITIALIZER;

void barrier(volatile int *count, int max)
{
    if (pthread_mutex_lock(&barrier_mut) != 0) {
        perror("pthread_mutex_lock");
        exit(-1);
    }

    (*count)++;

    if (*count == max) {
        if (pthread_cond_broadcast(&barrier_cond) != 0) {
            perror("pthread_cond_broadcast");
            exit(-1);
        }
    } else {
        do {
            if (pthread_cond_wait(&barrier_cond, &barrier_mut) != 0) {
                perror("pthread_cond_wait");
                exit(-1);
            }
        } while (*count < max);
    }

    if (pthread_mutex_unlock(&barrier_mut) != 0) {
        perror("pthread_mutex_unlock");
        exit(-1);
    }
}