#include <assert.h>
#include <inttypes.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define CAP 1024

#define da_append(da, item)                                                         \
    do {                                                                            \
        if ((da)->len >= (da)->cap) {                                               \
            (da)->cap = ((da)->cap == 0) ? CAP : (da)->cap * 2;                     \
            (da)->items = realloc((da)->items, sizeof(*(da)->items) * (da)->cap);   \
            assert((da)->items != NULL && "OOM!");                                  \
        }                                                                           \
        (da)->items[(da)->len++] = (item);                                          \
    } while (0)                                                                     \

typedef struct {
    uint8_t* items;
    size_t len;
    size_t cap;
    size_t ptr;
} Tape;

uint8_t tape_curr(Tape* tape) {
    return tape->items[tape->ptr];
}

void tape_assign(Tape* tape, uint8_t u8) {
    tape->items[tape->ptr] = u8;
}

void tape_shift(Tape* tape, int64_t delta) {
    int64_t ret = (int64_t)tape->ptr + delta;
    if (ret < 0) assert(0 && "Tape Underflow!");
    while ((size_t)ret >= tape->len) da_append(tape, 0);
    tape->ptr = (size_t)ret;
}

void tape_update(Tape* tape, int64_t delta) {
    tape->items[tape->ptr] += delta;
}

#define tape_jpf(tape, dst) if (tape_curr(tape) == 0) goto dst
#define tape_jpb(tape, dst) if (tape_curr(tape) != 0) goto dst

void tape_in(Tape* tape) {
    int c = fgetc(stdin);
    if (c != EOF) tape_assign(tape, c);
}

void tape_out(Tape* tape, size_t step) {
    for (size_t i = 0; i < step; ++i) {
        printf("%c", tape_curr(tape));
    }
}

void tape_add(Tape* tape, size_t pos) {
    tape->items[pos] += tape_curr(tape);
}

void tape_multiple(Tape* tape, size_t step) {
    tape_assign(tape, (uint8_t)(tape_curr(tape) * step));
}

void tape_init(Tape* tape) {
    for (size_t i = 0; i < CAP; ++i) {
        da_append(tape, 0);
    }
}

int main(void) {
    Tape tape = { 0 };
    tape_init(&tape);

    free(tape.items);
    return 0;
}