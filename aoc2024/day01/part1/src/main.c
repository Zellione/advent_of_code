#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

void cleanBuffer(char* buffer, int* current_index)
{
    for (int i = 0; i <= *current_index; i++)
    {
        buffer[i] = 0;
    }
    *current_index = 0;
}

int compare(const void* a, const void* b)
{
    return (*(int*)a - *(int*)b);
}

void sortArrays(int* left_side, int* right_side, int current_index)
{
    qsort(left_side, current_index, sizeof(int), compare);
    qsort(right_side, current_index, sizeof(int), compare);
}

int calculate(int* left_side, int* right_side, int current_index)
{
    int accumulated = 0;

    for (int i = 0; i <= current_index; i++)
    {
        int left  = left_side[i];
        int right = right_side[i];
        int delta = 0;

        if (left > right)
        {
            delta = left - right;
        }
        else
        {
            delta = right - left;
        }
        accumulated += delta;
    }

    return accumulated;
}

int main(int argc, char* argv[])
{
    int current_index    = 0;
    int left_side[1000]  = {0};
    int right_side[1000] = {0};

    FILE* fp;
    char* line;
    size_t len = 0;
    ssize_t read;

    fp = fopen("../../../day1_input", "r");

    if (fp == NULL)
        exit(EXIT_FAILURE);

    char buffer[50]   = {0};
    int buffer_index  = 0;
    bool is_left_side = true;
    while ((read = getline(&line, &len, fp)) != -1)
    {
        // printf("Retrieved line of length %zu:\n", read);
        // printf("Line: %s\n", line);
        // printf("%s\n", line);
        for (int i = 0; i < read; i++)
        {
            if (line[i] != ' ' && line[i] != '\n')
            {
                // printf("Current char: %i:%c\n", i, line[i]);
                buffer[buffer_index] = line[i];
                buffer_index++;
            }

            if ((line[i] == ' ' || line[i] == '\n') && buffer_index != 0)
            {
                printf("Buffer(%i): %s\n", buffer_index, buffer);
                if (is_left_side)
                {
                    // printf("Is left side\n");
                    // printf("%s\n", buffer);
                    left_side[current_index] = atoi(buffer);
                    i += 2;
                }
                else if (!is_left_side)
                {
                    // printf("Is right side\n");
                    // printf("%s\n", buffer);
                    right_side[current_index] = atoi(buffer);
                }

                if (i == read - 1)
                    current_index++;
                printf("(%i)(%zi)\n", i, (read - 1));
                is_left_side = is_left_side ? false : true;
                cleanBuffer(buffer, &buffer_index);
            }
        }
    }
    current_index--;

    fclose(fp);

    for (int i = 0; i <= current_index; i++)
    {
        printf("Left value: %i / Right Value %i\n", left_side[i], right_side[i]);
    }
    sortArrays(left_side, right_side, current_index + 1);
    printf("---\n");
    for (int i = 0; i <= current_index; i++)
    {
        printf("Left value: %i / Right Value %i\n", left_side[i], right_side[i]);
    }

    printf("\nThis is what I calculated: %i\n", calculate(left_side, right_side, current_index));

    return 0;
}
