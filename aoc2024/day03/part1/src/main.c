#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char* argv[])
{
    int accumulated = 0;

    FILE* fp;
    char line[1000] = {0};
    size_t len      = 0;
    ssize_t read;

    // fp = fopen("../../../day3_input", "r");
    fp = fopen("../../input_test", "r");

    if (fp == NULL)
        exit(EXIT_FAILURE);

    while (fgets(line, 1000, fp))
    {
        printf("\n%s", line);
    }
    fclose(fp);

    // printf("\nThis is what I calculated: %i\n", accumulated);

    return 0;
}
