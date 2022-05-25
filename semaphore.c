#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

#define NUM 10

void semaphore_acquire(volatile int *cnt) {
    for (;;) {
        while (*cnt >= NUM);
        __sync_fetch_and_add(cnt, 1);
        if (*cnt <= NUM) {
            break;
        }
        __sync_fetch_and_sub(cnt, 1);
    }
}

void semaphore_release(int *cnt) {
    __sync_fetch_and_sub(cnt, 1);
}

int cnt = 0;

void some_func() {
    for (int i = 0; i < NUM; i++) {
        semaphore_acquire(&cnt);
        cnt++;
        printf("%d\n", cnt);
        semaphore_release(&cnt);
    }
}

int main(int argc, char *argv[])
{
    some_func();
    return 0;
}