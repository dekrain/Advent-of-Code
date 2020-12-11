#include <stdio.h>
#include <stdlib.h>

short subtask1(char (*buf)[11], long lines);
void subtask2(char (*buf)[11], long lines, short max_id);

int main(void) {
    // Read the file
    FILE *f = fopen("./5.input", "r");
    fseek(f, 0, SEEK_END);
    long fsize = ftell(f);
    fseek(f, 0, SEEK_SET);
    // Portable file read
    char *buf = (char *)malloc(fsize);
    fread(buf, 1, fsize, f);
    fclose(f);

    // Get number of entries (10 chars + newline)
    long num_lines = fsize / 11;

    // Solve the subtask1 to get the max ID
    short max_id = subtask1((char (*)[11])buf, num_lines);

    // Finally solve the subtask2 and exit
    subtask2((char (*)[11])buf, num_lines, max_id);
    free(buf);
    return 0;
}

short subtask1(char (*buf)[11], long lines) {
    // Get max seat ID
    // Numbers are 10-bit
    // int16 is large enough
    short max_id = 0;
    for (long i = 0; i < lines; ++i) {
        char *entry = buf[i];

        // Parse entry
        short id = 0;
        for (int bit = 0; bit < 7; ++bit)
            id = (id << 1) | (*entry++ == 'B');
        for (int bit = 0; bit < 3; ++bit)
            id = (id << 1) | (*entry++ == 'R');

        // Compare & store
        if (id > max_id)
            max_id = id;
    }
    printf("Maximum seat ID is %d\n", max_id);
    return max_id;
}

void subtask2(char (*buf)[11], long lines, short max_id) {
    // Allocate memory
    // assuming bit-width of a byte is 8
    unsigned char *bitset = (unsigned char*)malloc((max_id + 7) >> 3);
    short min_id = max_id;
    for (long i = 0; i < lines; ++i) {
        char *entry = buf[i];

        // Parse entry
        short id = 0;
        for (int bit = 0; bit < 7; ++bit)
            id = (id << 1) | (*entry++ == 'B');
        for (int bit = 0; bit < 3; ++bit)
            id = (id << 1) | (*entry++ == 'R');

        // Store the bit in the set
        bitset[id >> 3] |= 1 << (id & 7);

        // Also track the min id for starting point
        if (id < min_id)
            min_id = id;
    }
    // Traverse the bitset to find missing seat ID
    // assuming the entry exists, so no bound-check
    for (short id = min_id; /*id < max_id*/; ++id) {
        if (!((1 << (id & 7)) & bitset[id >> 3])) {
            printf("Found missing ID %d\n", id);
            break;
        }
    }
    free(bitset);
}
