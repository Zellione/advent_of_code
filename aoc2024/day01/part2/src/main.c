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

int calculate(int* left_side, int* right_side, int current_index)
{
    int accumulated = 0;

    for (int i = 0; i <= current_index; i++)
    {
        int counter = 0;
        for (int j = 0; j <= current_index; j++)
        {
            if (left_side[i] == right_side[j])
            {
                counter++;
            }
        }

        accumulated += counter * left_side[i];
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
        for (int i = 0; i < read; i++)
        {
            if (line[i] != ' ' && line[i] != '\n')
            {
                buffer[buffer_index] = line[i];
                buffer_index++;
            }

            if ((line[i] == ' ' || line[i] == '\n') && buffer_index != 0)
            {
                if (is_left_side)
                {
                    left_side[current_index] = atoi(buffer);
                    i += 2;
                }
                else if (!is_left_side)
                {
                    right_side[current_index] = atoi(buffer);
                }

                if (i == read - 1)
                    current_index++;
                is_left_side = is_left_side ? false : true;
                cleanBuffer(buffer, &buffer_index);
            }
        }
    }
    current_index--;

    fclose(fp);

    for (int i = 0; i <= current_index; i++)
    {
        printf("l: %i / r: %i\n", left_side[i], right_side[i]);
    }

    printf("\nThis is what I calculated: %i\n\n", calculate(left_side, right_side, current_index));

    return 0;
}
