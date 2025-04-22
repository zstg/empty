#include <stdio.h>
#include <string.h>
typedef char* string;
#define or ||

int smart_append(string dst, string src) {
    /*
        c1 -> dest, c2 -> src
        Checks for available space before appending
        Appends as much as poss if there's not enough space.
        Always ensures that the buffer remains null-terminated
        Returns a success status if append was poss
    */
    if (c1 == NULL or c2 == NULL) { return 1;}
    const int BUFFER = 64;
    int src_len = strlen(src);
    int remaining_space = 64 - src_len +1; // +1 to accoomdate for \0
    
    char *ptr = c1;
    while (*ptr != '\0') {
        
    }
}

void concat(string c1, char *c2) {
    char *ptr = c1; // ptr stores the address of the first element of c1
    while(*ptr != '\0')
        ptr++; // automatically increments address by +1+ 4, since this is a character pointer

    // Now that we've reached the end of string c1, we have to add c2:
    while (*c2 != '\0') {
        *ptr = *c2; // for each byte available after the end of ptr copy a char from c2...
        ptr++; c2++;
    }
    *ptr = '\0';
}

int main() {
    char first[] = "Hi"; 
    // Since this is an in-place op, array needs to suff big +(at least 4B in this case, including the \0)+ in order to accomodate second. We thus need to allocate manually. But this is a pointer after all - out-of-bounds checks don't really do much... 
    printf("%d\n", sizeof(first)); // 2 chars -> 2*2B (+1 for \0)
    char second[] = " Stig!";
    printf("%d\n", sizeof(second)); // 6+1
    concat(first, &second); // GCC smort?
    printf("%d\n", sizeof(first));
    printf("%s\n", first);
}