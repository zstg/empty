#include <stdio.h>
#include <string.h>

void concat(char *c1, const char *c2) {
    // Move ptr to the end of the first string c1
    char *ptr = c1;
    while (*ptr != '\0') {
        ptr++;
    }

    // Now that we've reached the end of c1, we can concatenate c2
    while (*c2 != '\0') {
        *ptr = *c2;  // Copy each character from c2 to c1
        ptr++;       // Move to the next position in c1
        c2++;        // Move to the next character in c2
    }

    *ptr = '\0';  // Null-terminate the resulting string
}

int main() {
    char str1[100] = "Hello, "; // Ensure str1 is large enough
    char str2[] = "World!";
    
    concat(str1, str2);
    
    printf("%s\n", str1);  // Should print "Hello, World!"
    
    return 0;
}
