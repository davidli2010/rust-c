//
// Created by david on 2019/12/24.
//

#include "rust-c.h"
#include "c-api.h"
#include <stdio.h>
#include <assert.h>
#include <unistd.h>

sigjmp_buf *g_exception_stack = NULL;

#define TRY()  \
    do { \
        sigjmp_buf *save_exception_stack = g_exception_stack; \
        sigjmp_buf local_sigjmp_buf; \
        if (sigsetjmp(local_sigjmp_buf, 0) == 0) \
        { \
            g_exception_stack = &local_sigjmp_buf

#define CATCH()    \
        } \
        else \
        { \
            g_exception_stack = save_exception_stack;

#define END_TRY()  \
        } \
        g_exception_stack = save_exception_stack; \
} while (0)

void rethrow() {
    if (g_exception_stack != NULL) {
        printf("throw by jump\n");
        siglongjmp(*g_exception_stack, 1);
    } else {
        printf("rethrow with no exception stack\n");
        //assert(0);
    }
}

void print_from_c(const char *str) {
    printf("C: %s\n", str);
}

void longjmp_routine() {
    printf("long jump in C\n");
    rethrow();
}

void worker() {
    TRY();
    {
        worker_routine();
    }
    CATCH();
    {
        printf("exception happened in worker!\n");
    }
    END_TRY();
}

void WorkerMain() {
    sigjmp_buf local_sigjmp_buf;

    if (sigsetjmp(local_sigjmp_buf, 1) != 0) {
        printf("exception happened\n");
        return;
    }

    g_exception_stack = &local_sigjmp_buf;

    for (;;) {
        worker();
        usleep(1000000);
    }
}

int main() {
    print_hello_from_rust();

    uint32_t count = hm_chars("The tao of Rust");
    printf("%d\n", count);

    char *song = batman_song(5);
    printf("%s\n", song);
    free_song(song);

    uint32_t numbers[6] = {1, 2, 3, 4, 5, 6};
    uint32_t sum = sum_of_even(numbers, 6);
    printf("%d\n", sum);

    Tuple initial = {10, 20};
    Tuple new = flip_things_around(initial);
    printf("(%d,%d)\n", new.x, new.y);

    Database *database = database_new();
    database_insert(database);
    uint32_t pop1 = database_query(database, "10186");
    uint32_t pop2 = database_query(database, "10852");
    database_free(database);
    printf("%d\n", pop2 - pop1);

    print_str("call rust");

    WorkerMain();
}