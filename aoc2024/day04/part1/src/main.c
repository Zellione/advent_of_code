#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define ANSI_COLOR_RED "\x1b[31m"
#define ANSI_COLOR_RESET "\x1b[0m"

// #define TEST

#define WORD_LENGTH 4
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
int record_indices[WORD_LENGTH]            = {0};

void record_index(int index, int pos)
{
    record_indices[pos] = index;
}

void reset_indices()
{
    for (int i = 0; i < WORD_LENGTH; i++)
    {
        record_indices[i] = 0;
    }
}

void add_indices()
{
    for (int i = 0; i < WORD_LENGTH; i++)
    {
        indices_found[indices_index] = record_indices[i];
        indices_index++;
    }

    reset_indices();
}

bool is_found_index(int index)
{
    for (int i = 0; i <= indices_index; i++)
    {
        if (indices_found[i] == index)
            return true;
    }

    return false;
}

enum Directions
{
    LEFT,
    TOPLEFT,
    TOP,
    TOPRIGHT,
    RIGHT,
    BOTTOMRIGHT,
    BOTTOM,
    BOTTOMLEFT
};

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
    if (strstr(buffer, "XMAS") != NULL)
    {
        add_indices();
        return true;
    }
    reset_indices();
    return false;
}

int expand_right(char* data, int row, int col)
{
    if (!(col + WORD_LENGTH - 1 < ARRAY_COLS))
        return 0;

    char word[5] = {0};
    int index    = 0;

    for (int i = col; i < col + WORD_LENGTH; i++)
    {
        record_index(row * ARRAY_COLS + i, index);
        word[index] = data[row * ARRAY_COLS + i];
        index++;
    }
    word[WORD_LENGTH] = '\0';

    return is_xmas(word) ? 1 : 0;
}

int expand_left(char* data, int row, int col)
{
    if (!(col - (WORD_LENGTH - 1) >= 0))
        return 0;

    char word[5] = {0};
    int index    = 0;

    for (int i = col; i > col - WORD_LENGTH - 1; i--)
    {
        record_index(row * ARRAY_COLS + i, index);
        word[index] = data[row * ARRAY_COLS + i];
        index++;
    }
    word[WORD_LENGTH] = '\0';

    return is_xmas(word) ? 1 : 0;
}

int expand_top(char* data, int row, int col)
{
    if (!(row - (WORD_LENGTH - 1) >= 0))
        return 0;

    char word[5] = {0};
    int index    = 0;

    for (int i = row; i > row - WORD_LENGTH; i--)
    {
        record_index(i * ARRAY_COLS + col, index);
        word[index] = data[i * ARRAY_COLS + col];
        index++;
    }
    word[WORD_LENGTH] = '\0';

    return is_xmas(word) ? 1 : 0;
}

int expand_bottom(char* data, int row, int col)
{
    if (!(row + WORD_LENGTH - 1 < ARRAY_ROWS))
        return 0;

    char word[5] = {0};
    int index    = 0;

    for (int i = row; i < row + WORD_LENGTH; i++)
    {
        record_index(i * ARRAY_COLS + col, index);
        word[index] = data[i * ARRAY_COLS + col];
        index++;
    }
    word[WORD_LENGTH] = '\0';

    return is_xmas(word) ? 1 : 0;
}

int expand_top_left(char* data, int row, int col)
{
    // imagine row 9 col 3
    if (!(row - (WORD_LENGTH - 1) >= 0 && col - (WORD_LENGTH - 1) >= 0))
        return 0;

    char word[5] = {0};

    for (int i = 0; i < WORD_LENGTH; i++)
    {
        record_index((row - i) * ARRAY_COLS + (col - i), i);
        word[i] = data[(row - i) * ARRAY_COLS + (col - i)];
    }
    word[WORD_LENGTH] = '\0';

    return is_xmas(word) ? 1 : 0;
}

int expand_top_right(char* data, int row, int col)
{
    if (!(row - (WORD_LENGTH - 1) >= 0 && col + WORD_LENGTH - 1 < ARRAY_COLS))
        return 0;

    char word[5] = {0};

    for (int i = 0; i < WORD_LENGTH; i++)
    {
        record_index((row - i) * ARRAY_COLS + (col + i), i);
        word[i] = data[(row - i) * ARRAY_COLS + (col + i)];
    }
    word[WORD_LENGTH] = '\0';

    return is_xmas(word) ? 1 : 0;
}

int expand_bottom_right(char* data, int row, int col)
{
    if (!(row + WORD_LENGTH - 1 < ARRAY_ROWS && col + WORD_LENGTH - 1 < ARRAY_COLS))
        return 0;

    char word[5] = {0};

    for (int i = 0; i < WORD_LENGTH; i++)
    {
        record_index((row + i) * ARRAY_COLS + (col + i), i);
        word[i] = data[(row + i) * ARRAY_COLS + (col + i)];
    }
    word[WORD_LENGTH] = '\0';

    return is_xmas(word) ? 1 : 0;
}

int expand_bottom_left(char* data, int row, int col)
{
    if (!(row + WORD_LENGTH - 1 < ARRAY_ROWS && col - (WORD_LENGTH - 1) >= 0))
        return 0;

    char word[5] = {0};

    for (int i = 0; i < WORD_LENGTH; i++)
    {
        record_index((row + i) * ARRAY_COLS + (col - i), i);
        word[i] = data[(row + i) * ARRAY_COLS + (col - i)];
    }
    word[WORD_LENGTH] = '\0';

    return is_xmas(word) ? 1 : 0;
}

int expand(char* data, int row, int col, enum Directions dir)
{
    int words_found = 0;

    switch (dir)
    {
    case LEFT:
        words_found = expand_left(data, row, col);
        break;
    case TOPLEFT:
        words_found = expand_top_left(data, row, col);
        break;
    case TOP:
        words_found = expand_top(data, row, col);
        break;
    case TOPRIGHT:
        words_found = expand_top_right(data, row, col);
        break;
    case RIGHT:
        words_found = expand_right(data, row, col);
        break;
    case BOTTOMRIGHT:
        words_found = expand_bottom_right(data, row, col);
        break;
    case BOTTOM:
        words_found = expand_bottom(data, row, col);
        break;
    case BOTTOMLEFT:
        words_found = expand_bottom_left(data, row, col);
        break;
    }

    return words_found;
}

int check(char* data)
{
    int words_found = 0;

    for (int i = 0; i < ARRAY_ROWS; i++)
    {
        for (int j = 0; j < ARRAY_COLS; j++)
        {
            if (data[i * ARRAY_COLS + j] != 'X')
                continue;

            int temp = 0;
            for (int k = LEFT; k <= BOTTOMLEFT; k++)
            {
                words_found += expand(data, i, j, k);
            }
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
