#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Try rewriting like this:
// Search X and try to expand in all directions and check if chars are alligning and
// indices are still inbounds

// #define TEST
#define DEBUG

#define WORD_LENGTH 4
#ifndef TEST
#define ARRAY_ROWS 140
#define ARRAY_COLS 140
#define LINE_LENGTH 140
#define INPUT_FILE "../../../day4_input"
#else
#define ARRAY_ROWS 10
#define ARRAY_COLS 10
#define LINE_LENGTH 10
#define INPUT_FILE "../../input_test2"
#endif

#ifdef DEBUG
char tmp_test[ARRAY_COLS * ARRAY_ROWS] = {0};
int tmp_indices[WORD_LENGTH]           = {0};
#endif

void clean_buffer(char* buffer, int buffer_length)
{
    for (int i = 0; i < buffer_length; i++)
    {
        buffer[i] = 0;
    }
}

void read_data(char* line, char* data, int line_number)
{
    // printf("Buffer POS: %i %i %i\n", line_number, LINE_LENGTH, LINE_LENGTH * line_number);
    for (int i = 0; i < LINE_LENGTH; i++)
    {
        assert(line_number * LINE_LENGTH + i < ARRAY_COLS * ARRAY_ROWS);
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
#ifdef DEBUG
                tmp_indices[k] = i * LINE_LENGTH + j + k;
#endif
                tmp_buffer[k] = data[i * LINE_LENGTH + j + k];
            }

            tmp_buffer[WORD_LENGTH] = '\0';

            if (strstr(tmp_buffer, "XMAS") != NULL || strstr(tmp_buffer, "SAMX") != NULL)
            {
#ifdef DEBUG
                for (int k = 0; k < WORD_LENGTH; k++)
                    tmp_test[tmp_indices[k]] = tmp_buffer[k];
#endif
                words_found++;
            }

            clean_buffer(tmp_buffer, WORD_LENGTH + 1);
#ifdef DEBUG
            for (int k = 0; k < WORD_LENGTH; k++)
                tmp_indices[k] = 0;
#endif
        }
    }

    printf("Words found horizonal: %i\n", words_found);
    return words_found;
}

int check_vertical(char* data)
{
    char tmp_buffer[WORD_LENGTH + 1] = {0};
    int words_found                  = 0;

    for (int i = 0; i <= ARRAY_COLS; i++)
    {
        for (int j = 0; j <= ARRAY_ROWS - WORD_LENGTH; j++)
        {
            for (int k = 0; k < WORD_LENGTH; k++)
            {

#ifdef DEBUG
                tmp_indices[k] = (j + k) * LINE_LENGTH + i;
#endif
                tmp_buffer[k] = data[(j + k) * LINE_LENGTH + i];
            }

            tmp_buffer[WORD_LENGTH] = '\0';

            if (strstr(tmp_buffer, "XMAS") != NULL || strstr(tmp_buffer, "SAMX") != NULL)
            {
#ifdef DEBUG
                for (int k = 0; k < WORD_LENGTH; k++)
                    tmp_test[tmp_indices[k]] = tmp_buffer[k];
#endif
                words_found++;
            }

            clean_buffer(tmp_buffer, WORD_LENGTH + 1);
#ifdef DEBUG
            for (int k = 0; k < WORD_LENGTH; k++)
                tmp_indices[k] = 0;
#endif
        }
    }

    printf("Words found vertical: %i\n", words_found);
    return words_found;
}

int check_diagonal(char* data)
{
    char tmp_buffer[WORD_LENGTH + 1] = {0};
    int words_found                  = 0;
    int current_index                = 0;

    for (int i = 0; i <= ARRAY_ROWS - WORD_LENGTH; i++)
    {
        for (int j = 0; j <= ARRAY_COLS - WORD_LENGTH; j++)
        {
            for (int k = 0; k < WORD_LENGTH; k++)
            {
#ifdef DEBUG
                tmp_indices[k] = (i + k) * LINE_LENGTH + (j + k);
#endif
                tmp_buffer[k] = data[(i + k) * LINE_LENGTH + (j + k)];
            }
            tmp_buffer[WORD_LENGTH] = '\0';

            if (strstr(tmp_buffer, "XMAS") != NULL || strstr(tmp_buffer, "SAMX") != NULL)
            {
#ifdef DEBUG
                for (int k = 0; k < WORD_LENGTH; k++)
                    tmp_test[tmp_indices[k]] = tmp_buffer[k];
#endif
                words_found++;
            }

            clean_buffer(tmp_buffer, WORD_LENGTH + 1);
#ifdef DEBUG
            for (int k = 0; k < WORD_LENGTH; k++)
                tmp_indices[k] = 0;
#endif
        }
    }

    for (int i = 0; i <= ARRAY_ROWS - WORD_LENGTH; i++)
    {
        for (int j = ARRAY_COLS; j >= WORD_LENGTH; j--)
        {
            for (int k = 0; k < WORD_LENGTH; k++)
            {
#ifdef DEBUG
                tmp_indices[k] = ((i + k) * LINE_LENGTH) - 1 + (j - k);
#endif
                tmp_buffer[k] = data[((i + k) * LINE_LENGTH) - 1 + (j - k)];
            }
            tmp_buffer[WORD_LENGTH] = '\0';

            if (strstr(tmp_buffer, "XMAS") != NULL || strstr(tmp_buffer, "SAMX") != NULL)
            {
#ifdef DEBUG
                for (int k = 0; k < WORD_LENGTH; k++)
                    tmp_test[tmp_indices[k]] = tmp_buffer[k];
#endif
                words_found++;
            }

#ifdef DEBUG
            for (int k = 0; k < WORD_LENGTH; k++)
                tmp_indices[k] = 0;
#endif
            clean_buffer(tmp_buffer, WORD_LENGTH + 1);
        }
    }

    printf("Words found diagonal: %i\n", words_found);
    return words_found;
}

int check(char* data)
{
    int words_found = 0;

    words_found += check_horizontal(data);
    // words_found += check_vertical(data); words_found += check_diagonal(data);

    return words_found;
}

int main(int argc, char* argv[])
{
    uint accumulated = 0;

    FILE* fp;
    char line[LINE_LENGTH + 1] = {0};
    ssize_t read;

    char data[ARRAY_COLS * ARRAY_ROWS];

    fp = fopen(INPUT_FILE, "r");

    if (fp == NULL)
        exit(EXIT_FAILURE);

    int buffer_pos = 0;
    while (fgets(line, LINE_LENGTH + 1, fp))
    {
        if (line[0] == '\n')
            continue;
        read_data(line, data, buffer_pos);
        buffer_pos++;
    }
    fclose(fp);

#ifdef DEBUG
    for (int i = 0; i < ARRAY_ROWS; i++)
        for (int j = 0; j < ARRAY_COLS; j++)
            tmp_test[i * LINE_LENGTH + j] = '.';
#endif

    accumulated += check(data);
    print_data(data);
#ifdef DEBUG
    printf("\n");
    print_data(tmp_test);
#endif

    printf("\nThis is what I calculated: %i\n", accumulated);

    return 0;
}
