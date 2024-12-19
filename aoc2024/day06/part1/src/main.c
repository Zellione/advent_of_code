#include <assert.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>

#define ANSI_COLOR_RED "\x1b[31m"
#define ANSI_COLOR_RESET "\x1b[0m"

// #define TEST

#ifndef TEST
#define GRID_COLS 130
#define GRID_ROWS 130
#define INPUT_FILE "../../../day6_input"
#else
#define GRID_COLS 10
#define GRID_ROWS 10
#define INPUT_FILE "../../input_test"
#endif

typedef __uint32_t u32;
typedef __int32_t i32;

enum Direction
{
    UP,
    RIGHT,
    LEFT,
    DOWN
};

void create_grid_row(char* line, i32 row, char* grid)
{
    for (i32 i = 0; i < GRID_COLS; i++)
    {
        grid[row * GRID_COLS + i] = line[i];
    }
}

void display_grid(const char* grid)
{
    i32 pos = 0;
    printf("\n");
    for (i32 i = 0; i < GRID_ROWS; i++)
    {
        for (i32 j = 0; j < GRID_ROWS; j++)
        {
            pos = i * GRID_COLS + j;
            if (grid[pos] == '*')
            {
                printf(ANSI_COLOR_RED "%c" ANSI_COLOR_RESET, grid[pos]);
            }
            else
            {
                printf("%c", grid[pos]);
            }
        }
        printf("\n");
    }
    printf("\n");
}

void find_starting_position(char* grid, i32* start_row, i32* start_col)
{
    for (i32 i = 0; i < GRID_ROWS; i++)
    {
        for (i32 j = 0; j < GRID_ROWS; j++)
        {
            if (grid[i * GRID_COLS + j] == '^')
            {
                *start_row = i;
                *start_col = j;
                return;
            }
        }
    }
}

bool is_inbounds(i32 curr_row, i32 curr_col)
{
    return curr_row >= 0 && curr_row < GRID_ROWS && curr_col >= 0 && curr_col < GRID_COLS;
}

bool is_cached(i32* cache, i32 pos, i32 cache_pos)
{
    for (i32 i = 0; i < cache_pos; i++)
    {
        if (cache[i] == pos)
            return true;
    }
    return false;
}

u32 simulate_path(char* grid)
{
    i32 cache[10000]   = {0};
    i32 cache_pos      = 0;
    enum Direction dir = UP;
    u32 accumulated    = 0;
    i32 curr_row       = 0;
    i32 curr_col       = 0;
    find_starting_position(grid, &curr_row, &curr_col);
    assert(curr_row > 0 && curr_col > 0);

    i32 curr_pos = 0;
    i32 next_pos = 0;
    while (is_inbounds(curr_row, curr_col))
    {
        curr_pos = curr_row * GRID_COLS + curr_col;
        switch (dir)
        {
        case UP:
            next_pos = (curr_row - 1) * GRID_COLS + curr_col;
            dir      = grid[next_pos] == '#' ? RIGHT : dir;
            curr_row = dir == UP ? curr_row - 1 : curr_row;
            break;
        case RIGHT:
            next_pos = curr_row * GRID_COLS + curr_col + 1;
            dir      = grid[next_pos] == '#' ? DOWN : dir;
            curr_col = dir == RIGHT ? curr_col + 1 : curr_col;
            break;
        case LEFT:
            next_pos = curr_row * GRID_COLS + curr_col - 1;
            dir      = grid[next_pos] == '#' ? UP : dir;
            curr_col = dir == LEFT ? curr_col - 1 : curr_col;
            break;
        case DOWN:
            next_pos = (curr_row + 1) * GRID_COLS + curr_col;
            dir      = grid[next_pos] == '#' ? LEFT : dir;
            curr_row = dir == DOWN ? curr_row + 1 : curr_row;
            break;
        }
        if (!is_cached(cache, curr_pos, cache_pos))
        {
            accumulated++;
            cache[cache_pos] = curr_pos;
            cache_pos++;
        }
        grid[curr_pos] = '*';
    }

    return accumulated;
}

i32 main(i32 argc, char* argv[])
{
    u32 accumulated = 0;

    FILE* fp;
    char line[1000] = {0};
    char grid[GRID_COLS * GRID_ROWS];

    fp = fopen(INPUT_FILE, "r");

    if (fp == NULL)
        exit(EXIT_FAILURE);

    i32 row = 0;
    while (fgets(line, 1000, fp))
    {
        create_grid_row(line, row, grid);
        row++;
    }
    fclose(fp);

    display_grid(grid);
    accumulated = simulate_path(grid);
    display_grid(grid);

    printf("\nThis is what I calculated: %i\n", accumulated);

    return 0;
}
