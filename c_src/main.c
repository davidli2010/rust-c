//
// Created by david on 2019/12/24.
//

#include "rust-c.h"
#include "c-api.h"
#include <stdio.h>

void print_from_c(const char* str) {
    printf("C: %s\n", str);
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
}