#include <stdio.h>
#include <stdlib.h>
#include "Basics.c"

void update_coord(Coordinate coord, int new_x) {
    coord.x = new_x;
}

// Structs are passed by value, not by reference
Coordinate update_and_return_coord(Coordinate coord, int new_x) {
    coord.x = new_x;
    return coord;
}
// Here's how to make the above change in place:
Coordinate update_and_return_coord_inplace(Coordinate *coord, int new_x) {
    (*coord).x = new_x;
    return *coord;
}


// out-of-place, but (*pointer).field== pointer->field. Both aren't the same as *pointer.field

int main() {
/*
    int x = 5;
    int y = x;

    printf("Address of x: %p Address of y: %p\n", &x, &y);
    int *z = &x;
    // Needless to say, z is a "pointer" that stores the address of x
    printf("Address of x: %p Address of z: %p\n", &x, z);
    unsigned long z2 = z; // this is a COPY of that address z; a mere example to show that pointers are addresses after all. The `unsigned long` shenanigan is to print the hex value including the 7fff crap
    printf("CONTENT of z2: 0x%lx\n", z2);
*/
    Coordinate c = { .x = 12, .y = 23, .z = 34 };
    Coordinate newc = update_and_return_coord_inplace(&c, 13);
    printf("%d\n", newc.x);
    printf("%d\n", c.x); // this sucker gets updated TOO - call by reference!
}