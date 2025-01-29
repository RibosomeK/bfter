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

typedef struct {
    char* items;
    size_t len;
    size_t cap;
} InputBuf;

uint8_t tape_curr(Tape* tape) {
    return tape->items[tape->ptr];
}

void tape_asign(Tape* tape, uint8_t u8) {
    tape->items[tape->ptr] = u8;
}

void tape_shift(Tape* tape, int64_t delta) {
    int64_t ret = (int64_t)tape->ptr + delta;
    if (ret < 0) assert(0 && "Tape Underflow!");
    if ((size_t)ret >= tape->len) {
        printf("WARN: Possible Tape Overflow!\n");
        printf("WARN: Current tape pointer: %zu", tape->ptr);
    }
    while ((size_t)ret >= tape->len) {
        da_append(tape, 0);
    }
    tape->ptr = (size_t)ret;
}

void tape_update(Tape* tape, int64_t delta) {
    tape->items[tape->ptr] += delta;
}

#define tape_jpf(tape, dst) if (tape_curr(tape) == 0) goto dst
#define tape_jpb(tape, dst) if (tape_curr(tape) != 0) goto dst

void tape_in(Tape* tape) {
    printf("Please input a number (0-255) or a ascii char: ");
    InputBuf buf = { 0 };
    char* endptr;
    uint8_t ret;
    while (true) {
        char c = fgetc(stdin);
        if (c == EOF || c == '\n') break;
        da_append(&buf, c);
    }
    da_append(&buf, '\0');
    if (buf.items[0] == '\0') assert(0 && "Invalid input: Empty String!");
    uint8_t num = strtol(buf.items, &endptr, 10);
    if (buf.items == endptr) tape_asign(tape, (uint8_t)buf.items[0]);
    else tape_asign(tape, num);
    free(buf.items);
}

void tape_out(Tape* tape, size_t step) {
    for (size_t i = 0; i < step; ++i) {
        printf("%c", tape_curr(tape));
    }
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