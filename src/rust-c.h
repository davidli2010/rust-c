//
// Created by david on 2019/12/24.
//

#ifndef CALLRUST_H
#define CALLRUST_H

#include <inttypes.h>
#include <stdio.h>

void print_hello_from_rust();
uint32_t hm_chars(const char *str);
char * batman_song(uint8_t length);
void free_song(char *);
uint32_t sum_of_even(const uint32_t *numbers, size_t length);

typedef struct {
    uint32_t x;
    uint32_t y;
} tuple_t;

tuple_t flip_things_around(tuple_t);

typedef struct DatabaseSt Database;

Database * database_new();
void database_free(Database*);
void database_insert(Database*);
uint32_t database_query(const Database*, const char *zip);

void print_str(const char *str);

#endif //CALLRUST_H
