#include "spinlock.c"

#include <assert.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

struct reent_lock {
    bool lock;
    int id;
    int cnt;
};

void reentlock_acquire(struct reent_lock *lock, int id) {
    if (lock->lock && __atomic_load_n(&lock->id, __ATOMIC_RELAXED) == id) {
        lock->cnt++;
    } else {
        spinlock_acquire(&lock->lock);
        __atomic_store_n(&lock->id, id, __ATOMIC_RELAXED);
        lock->cnt++;
    }
}

void reentlock_release(struct reent_lock *lock) {
    lock->cnt--;
    if (lock->cnt == 0) {
        __atomic_store_n(&lock->id, 0, __ATOMIC_RELAXED);
        spinlock_release(&lock->lock);
    }
}

struct reent_lock lock_var;

void reent_lock_test(int id, int n) {
    if (n==0) return;

    reentlock_acquire(&lock_var, id);
    reent_lock_test(id, n-1);
    reentlock_release(&lock_var);
}

void *thread_func(void *arg) {
    int id = (int)arg;
    assert(id != 0);
    for (int i=0; i < 10000; i++) {
        reent_lock_test(id, 10);
    }
    return NULL;
}

int main(int argc, char *argv[]) {
    pthread_t v[NUM_THREADS];
    for (int i=0; i < NUM_THREADS; i++) {
        pthread_create(&v[i], NULL, thread_func, (void *)(i+1));
    }
    for (int i=0; i < NUM_THREADS; i++) {
        pthread_join(v[i], NULL);
    }
    return 0;
}