#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define WORD_LENGTH 4
// #define ARRAY_ROWS 140
// #define ARRAY_COLS 140
#define ARRAY_ROWS 10
#define ARRAY_COLS 10
#define LINE_LENGTH 10

void clean_buffer(char* buffer, int buffer_length)
{
    for (int i = 0; i < buffer_length; i++)
    {
        buffer[i] = 0;
    }
}

int check_horizontal(char* data)
{
    char tmp_buffer[WORD_LENGTH + 1] = {0};
    int words_found                  = 0;
    for (int i = 0; i < ARRAY_ROWS; i++)
    {
        for (int j = 0; j < ARRAY_COLS - WORD_LENGTH; j++)
        {
            for (int k = 0; k < WORD_LENGTH; k++)
            {
                tmp_buffer[k] = data[i * LINE_LENGTH + j + k];
            }
        }
        tmp_buffer[WORD_LENGTH] = '\0';

        if (strstr(tmp_buffer, "XMAS") != NULL || strstr(tmp_buffer, "SAMX") != NULL)
        {
            words_found++;
        }

        clean_buffer(tmp_buffer, WORD_LENGTH + 1);
    }

    return words_found;
}

int check_vertical(char* data)
{
    char tmp_buffer[WORD_LENGTH + 1] = {0};
    int words_found                  = 0;
    for (int i = 0; i < ARRAY_COLS - WORD_LENGTH; i++)
    {
        for (int j = 0; j < ARRAY_ROWS; j++)
        {
            for (int k = 0; k < WORD_LENGTH; k++)
            {
                tmp_buffer[k] = data[j * LINE_LENGTH + i + k];
            }
        }
        tmp_buffer[WORD_LENGTH] = '\0';

        if (strstr(tmp_buffer, "XMAS") != NULL || strstr(tmp_buffer, "SAMX") != NULL)
        {
            words_found++;
        }

        clean_buffer(tmp_buffer, WORD_LENGTH + 1);
    }

    return words_found;
}

int check(char* data)
{
    int words_found = 0;

    words_found += check_horizontal(data);
    words_found += check_vertical(data);

    return words_found;
}

void read_data(char* line, char* data, int line_number)
{
    // printf("Buffer POS: %i %i %i\n", line_number, LINE_LENGTH, LINE_LENGTH * line_number);
    for (int i = 0; i < LINE_LENGTH; i++)
    {
        // assert(line_number * LINE_LENGTH + i < 100);
        data[line_number * LINE_LENGTH + i] = line[i];
    }
}

void print_data(char* data)
{
    for (int i = 0; i < ARRAY_ROWS; i++)
    {
        for (int j = 0; j < ARRAY_COLS; j++)
        {
            // printf("i: %i; j: %i index: %i\n", i, j, i * LINE_LENGTH + j);
            printf("%c", data[i * LINE_LENGTH + j]);
        }
        printf("\n");
    }
}

int main(int argc, char* argv[])
{
    uint accumulated = 0;

    FILE* fp;
    char line[LINE_LENGTH + 1] = {0};
    ssize_t read;

    char data[ARRAY_COLS * ARRAY_ROWS];

    // fp = fopen("../../../day4_input", "r");
    fp = fopen("../../input_test", "r");

    if (fp == NULL)
        exit(EXIT_FAILURE);

    int buffer_pos = 0;
    while (fgets(line, LINE_LENGTH + 1, fp))
    {
        if (line[0] == '\n')
            continue;
        read_data(line, data, buffer_pos);
        accumulated += check(data);
        buffer_pos++;
    }
    fclose(fp);

    print_data(data);

    printf("\nThis is what I calculated: %i\n", accumulated);

    return 0;
}
