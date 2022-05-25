#define HOLDTIME 10

void do_lock() {
    for (uint64_t i = 0; i < HOLDTIME; i++) {
        asm volatile("nop");
    }
}