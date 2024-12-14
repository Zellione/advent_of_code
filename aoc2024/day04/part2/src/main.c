#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define ANSI_COLOR_RED "\x1b[31m"
#define ANSI_COLOR_RESET "\x1b[0m"

// #define TEST

#define WORD_LENGTH 3
#ifndef TEST
#define ARRAY_ROWS 140
#define ARRAY_COLS 140
#define INPUT_FILE "../../../day4_input"
#else
#define ARRAY_ROWS 10
#define ARRAY_COLS 10
#define INPUT_FILE "../../input_test"
#endif

int indices_found[ARRAY_ROWS * ARRAY_COLS] = {0};
int indices_index                          = 0;
int record_indices[WORD_LENGTH * 2]        = {0};
int record_index_pos                       = 0;

void record_index(int index)
{
    assert(record_index_pos < WORD_LENGTH * 2);
    record_indices[record_index_pos] = index;
    record_index_pos++;
}

void reset_indices()
{
    for (int i = 0; i < WORD_LENGTH; i++)
    {
        record_indices[i] = 0;
    }
    record_index_pos = 0;
}

void add_indices()
{
    for (int i = 0; i < WORD_LENGTH * 2; i++)
    {
        indices_found[indices_index] = record_indices[i];
        indices_index++;
    }

    reset_indices();
}

bool is_found_index(int index)
{
    for (int i = 0; i < indices_index; i++)
    {
        if (indices_found[i] == index)
            return true;
    }

    return false;
}

void clean_buffer(char* buffer, int buffer_length)
{
    for (int i = 0; i < buffer_length; i++)
    {
        buffer[i] = 0;
    }
}

void read_data(char* line, char* data, int line_number)
{
    for (int i = 0; i < ARRAY_COLS; i++)
    {
        assert(line_number * ARRAY_COLS + i < ARRAY_COLS * ARRAY_ROWS);
        data[line_number * ARRAY_COLS + i] = line[i];
    }
}

void print_data(char* data)
{
    for (int i = 0; i < ARRAY_ROWS; i++)
    {
        for (int j = 0; j < ARRAY_COLS; j++)
        {
            int temp = i * ARRAY_COLS + j;
            if (is_found_index(temp))
            {
                printf(ANSI_COLOR_RED "%c" ANSI_COLOR_RESET, data[temp]);
            }
            else
            {
                printf("%c", data[temp]);
            }
        }
        printf("\n");
    }
}

bool is_xmas(char* buffer)
{
    if (strstr(buffer, "MAS") != NULL || strstr(buffer, "SAM"))
    {
        return true;
    }
    return false;
}

bool expand_up_left_bottom_right(char* data, int row, int col)
{
    if (!(row - 1 > 0 && col + 1 < ARRAY_COLS) && !(row + 1 < ARRAY_ROWS && col + 1 < ARRAY_ROWS))
        return 0;

    char word[4] = {0};

    int start_row = row - 1;
    int start_col = col - 1;
    for (int i = 0; i < WORD_LENGTH; i++)
    {
        int cur_index = (start_row + i) * ARRAY_COLS + (start_col + i);
        record_index(cur_index);
        word[i] = data[cur_index];
    }
    word[WORD_LENGTH] = '\0';

    return is_xmas(word);
}

bool expand_up_right_bottom_left(char* data, int row, int col)
{
    if (!(row - 1 >= 0 && col + 1 < ARRAY_COLS) && !(row + 1 < ARRAY_ROWS && col - 1 >= 0))
        return 0;

    char word[4] = {0};

    int start_row = row - 1;
    int start_col = col + 1;
    for (int i = 0; i < WORD_LENGTH; i++)
    {
        int cur_index = (start_row + i) * ARRAY_COLS + (start_col - i);
        record_index(cur_index);
        word[i] = data[cur_index];
    }
    word[WORD_LENGTH] = '\0';

    return is_xmas(word);
}

int expand(char* data, int row, int col)
{
    int words_found = 0;

    if (expand_up_left_bottom_right(data, row, col) && expand_up_right_bottom_left(data, row, col))
    {
        add_indices();
        words_found = 1;
    }
    reset_indices();

    return words_found;
}

int check(char* data)
{
    int words_found = 0;

    for (int i = 1; i < ARRAY_ROWS; i++)
    {
        for (int j = 1; j < ARRAY_COLS; j++)
        {
            if (data[i * ARRAY_COLS + j] != 'A')
                continue;

            words_found += expand(data, i, j);
        }
    }

    return words_found;
}

int main(int argc, char* argv[])
{
    uint accumulated = 0;

    FILE* fp;
    char line[ARRAY_COLS + 1] = {0};
    ssize_t read;

    char data[ARRAY_COLS * ARRAY_ROWS];

    fp = fopen(INPUT_FILE, "r");

    if (fp == NULL)
        exit(EXIT_FAILURE);

    int buffer_pos = 0;
    while (fgets(line, ARRAY_COLS + 1, fp))
    {
        if (line[0] == '\n')
            continue;
        read_data(line, data, buffer_pos);
        buffer_pos++;
    }
    fclose(fp);

    accumulated += check(data);

    print_data(data);
    printf("\nThis is what I calculated: %i\n", accumulated);

    return 0;
}
