#include <pthread.h>

#define HOLDTIME 10

pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;
void do_lock() {
    pthread_mutex_lock(&lock);
    for (uint64_t i = 0; i < HOLDTIME; i++) {
        asm volatile("nop");
    }
    pthread_mutex_unlock(&lock);
}