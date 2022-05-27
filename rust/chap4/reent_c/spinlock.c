#include "tas.c"

void spinlock_acquire(volatile bool *lock) {
    for (;;) {
        while (*lock);
        if (!test_and_set(lock)) {
            break;
        }
    }
}

void spinlock_release(bool *lock) {
    tas_release(lock);
}