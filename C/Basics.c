#include <stdio.h>

typedef struct { // don't NEED to add `coordinate` here
    int x, y, z;
} Coordinate;

// Now write a function that operates on these structs

Coordinate scale_coordinate(Coordinate coord, int scale_factor){
    Coordinate scaled = {
        .x = coord.x*scale_factor,
        .y = coord.y*scale_factor,
        .z = coord.z*scale_factor,
    };
    return scaled;
}

char *printTemp(int temp) {
    if (temp < 70) {
        return "too cold";
    } else {
        return "IDC";
    }
}

unsigned long int structSize(Coordinate c) {
    return sizeof(c.x) + sizeof(c.y) + sizeof(c.z);
}

/*
int main() {
    Coordinate c = { .x = 12, .y = 23, .z = 34 };
    Coordinate result = scale_coordinate(c, 17);
    printf("X = %d Y = %d Z = %d\n", c.x, c.y, c.z);
    // Obviously sizeof(result) = sizeof(c)
    printf("Struct size: %ld\n", sizeof(result));
    printf("Struct members' size: %ld\n", structSize(result));

}
*/    