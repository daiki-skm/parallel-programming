#include <inttypes.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#include "rwlock.c" // c1
//#include "rwlock_wr.c"
//#include "mutex.c"
//#include "empty.c"
#include "barrier.c"

#define NUM_THREAD 10

volatile int flag = 0;

volatile int waiting_1 = 0;
volatile int waiting_2 = 0;

uint64_t count[NUM_THREAD - 1]; // c2

void *worker(void *arg) { // c3
    uint64_t id = (uint64_t)arg;
    barrier(&waiting_1, NUM_THREAD);

    uint64_t n = 0; // c4
    while (flag == 0) {
        do_lock(); // c5
        n++;
    }
    count[id] = n;

    barrier(&waiting_2, NUM_THREAD);

    return NULL;
}

void *timer(void *arg) { // c6
    barrier(&waiting_1, NUM_THREAD);

    sleep(180);
    flag = 1;

    barrier(&waiting_2, NUM_THREAD);
    for (int i = 0; i < NUM_THREAD - 1; i++) {
        printf("%llu\n", count[i]);
    }

    return NULL;
}

int main() {
    for (uint64_t i = 0; i < NUM_THREAD - 1; i++) {
        pthread_t th;
        pthread_create(&th, NULL, worker, (void *)i);
        pthread_detach(th);
    }

    pthread_t th;
    pthread_create(&th, NULL, timer, NULL);
    pthread_join(th, NULL);

    return 0;
}