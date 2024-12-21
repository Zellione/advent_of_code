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

typedef enum
{

    UP,
    RIGHT,
    DOWN,
    LEFT,
    DIRECTION_MAX,
} Direction;

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

bool is_bounced(i32* cache, i32 pos, i32 cache_pos)
{
    i32 bounce_count = 0;
    for (i32 i = 0; i < cache_pos; i++)
    {
        if (cache[i] == pos)
            bounce_count++;

        if (bounce_count > 2)
            return true;
    }
    return false;
}

void copy_grid(const char* source, char* target)
{
    for (i32 i = 0; i < GRID_ROWS; i++)
    {
        for (i32 j = 0; j < GRID_ROWS; j++)
        {
            target[i * GRID_COLS + j] = source[i * GRID_COLS + j];
        }
    }
}

bool can_move(char* grid, i32 row, i32 col, Direction dir)
{
    switch (dir)
    {
    case UP:
        return row == 0 || (row > 0 && grid[(row - 1) * GRID_COLS + col] != '#');
    case RIGHT:
        return col == (GRID_COLS - 1) || (col < (GRID_COLS - 1) && grid[row * GRID_COLS + (col + 1)] != '#');
    case DOWN:
        return row == (GRID_ROWS - 1) || (row < (GRID_ROWS - 1) && grid[(row + 1) * GRID_COLS + col] != '#');
    case LEFT:
        return col == 0 || (col > 0 && grid[row * GRID_COLS + (col - 1)] != '#');
    default:
        assert(0 && "Not recognized direction");
    }
}

i32 CLAMP_DIRECTION(int x, int n)
{
    return ((x % n) + n) % n;
}

void turn_right(char* grid, i32 row, i32 col, Direction* dir)
{
    Direction temp_dir = *dir;

    *dir = CLAMP_DIRECTION(temp_dir + 1, DIRECTION_MAX);
}

void move(char* grid, i32* row, i32* col, Direction dir)
{
    if (grid[*row * GRID_COLS + *col] != '^')
        grid[*row * GRID_COLS + *col] = '*';
    switch (dir)
    {
    case UP:
        *row -= 1;
        break;
    case RIGHT:
        *col += 1;
        break;
    case LEFT:
        *col -= 1;
        break;
    case DOWN:
        *row += 1;
        break;
    default:
        assert(0 && "THIS CASE SHOULD NEVER HAPPEN");
        break;
    }
}

bool go_forward(char* grid, i32* row, i32* col, Direction* dir)
{
    while (!can_move(grid, *row, *col, *dir))
        turn_right(grid, *row, *col, dir);
    move(grid, row, col, *dir);

    return *col >= GRID_COLS || *row >= GRID_ROWS || *col < 0 || *row < 0;
}

i32 simulate_path(char* grid, i32 replacement_pos)
{
    i32 result      = 0;
    i32 steps_taken = 0;

    Direction dir = UP;
    i32 curr_row  = 0;
    i32 curr_col  = 0;
    find_starting_position(grid, &curr_row, &curr_col);
    assert(curr_row > 0 && curr_col > 0);

    while (!go_forward(grid, &curr_row, &curr_col, &dir) && steps_taken < GRID_ROWS * GRID_COLS)
        steps_taken++;

    if (steps_taken == GRID_ROWS * GRID_COLS)
        result = 1;

    return result;
}

i32 main(i32 argc, char* argv[])
{
    i32 accumulated = 0;

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
    i32 curr_pos = 0;

    char working_grid[GRID_ROWS * GRID_COLS] = {0};
    for (int i = 0; i < GRID_ROWS; i++)
    {
        for (int j = 0; j < GRID_ROWS; j++)
        {
            curr_pos = i * GRID_COLS + j;
            if (grid[curr_pos] == '^' || grid[curr_pos] == '#')
                continue;
            copy_grid(grid, working_grid);
            working_grid[curr_pos] = '#';
            accumulated += simulate_path(working_grid, curr_pos);
        }
    }

    printf("\nThis is what I calculated: %i\n", accumulated);

    return 0;
}
