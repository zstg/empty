#include "Basics.c"

// IDK why this array is defined globally...
Coordinate CoordArrays[3] = {
    {5, 4, 1},
    {7, 3, 2},
    {9, 6, 8}
};

int main(){
    // Arrays decay to pointers. Technically speaking an array is just the address of the first element. There's no way to actually define the address of the last element or the size of the array. C does NOT have out-of-bounds checking.
    int arr[5] = {23,34,45,56,67};
    int *ptr = arr; // this means ptr simply stores the address of the first element of the array...
    printf("%d %d 5d\n", *ptr+2, *(ptr+2), *(arr+2));
    // since ptr stores the address of the first element, *(ptr)+2 returns 25. *(ptr+2) is the content of the "address pointed to by ptr + 2".
    // we know that `arr' is the address of the first element. Thus the content of *(arr+2) is the content of some (random address inc by 2) - it's arbitrary.

    int *startingPoint =  CoordArrays; //  the starting address of the array i.e the address of the first element (i.e the addr of "5"). Note that technically you're supposed to typecast...
    for(int i=0; i<9; i++)
        printf("CoordArrays[%d] = %d\n", i, *(startingPoint + i));// obviously startingPoint[i] also works
}