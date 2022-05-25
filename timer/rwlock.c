#include <pthread.h>

#define HOLDTIME 10

pthread_rwlock_t lock = PTHREAD_RWLOCK_INITIALIZER;
void do_lock() {
    pthread_rwlock_rdlock(&lock);
    for (uint64_t i = 0; i < HOLDTIME; i++) {
        asm volatile("nop");
    }
    pthread_rwlock_unlock(&lock);
}