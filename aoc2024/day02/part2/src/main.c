#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

int parse_input(const char* line, const int line_length, int* numbers, const int length)
{
    int numbers_parsed = 0;
    int buffer_index   = 0;
    char buffer[3];

    for (int i = 0; i < line_length; i++)
    {
        if (line[i] != ' ' && line[i] != '\n' & line[i] != '\0')
        {
            assert(buffer_index < 3);

            buffer[buffer_index] = line[i];
            buffer_index++;
        }
        else
        {
            if (numbers_parsed == length || line[i] == '\0')
                break;

            buffer[buffer_index + 1] = '\0';
            int tmp                  = atoi(buffer);

            numbers[numbers_parsed]  = tmp;
            numbers_parsed++;

            for (int j = 0; j < buffer_index; j++)
            {
                buffer[j] = 0;
            }
            buffer_index = 0;
        }
    }

    return numbers_parsed;
}

/**
 * It was obvious but we need to brute force check if by removing all falty indexes
 */
int check(int* numbers, int length)
{
    for (int i = 0; i < length; i++)
    {
        printf("%i\t", numbers[i]);
    }
    printf("\n");
    int direction = 0;
    for (int i = 0; i < length - 1; i++)
    {
        int delta = numbers[i] - numbers[i + 1];

        if (direction == 0)
            direction = delta > 0 ? 1 : -1;

        int udelta = abs(delta);
        // printf("\tDelta: %i Direction: %i Current: %i Previous %i\n", delta, direction, numbers[i + 1], numbers[i]);
        if (udelta > 3 || udelta < 1 || (delta < 0 && direction == 1) || (delta > 0 && direction == -1))
        {
            printf("... fail\n");
            return i;
        }
    }

    printf("... pass\n");
    return -1;
}

void cpy_array(int* source, int* target, const int length)
{
    for (int i = 0; i < length; i++)
    {
        target[i] = source[i];
    }
}

int remove_element(int* numbers, int length, int element)
{
    int tmp_nums[10];
    for (int i = 0; i < length; i++)
    {
        int tmp_index = i;
        if (i >= element)
            tmp_index += 1;

        tmp_nums[i] = numbers[tmp_index];
    }

    for (int i = 0; i < length - 1; i++)
    {
        numbers[i] = tmp_nums[i];
    }

    return length - 1;
}

bool try_all_faulty_indices(int* numbers, int length)
{
    int tmp_numbers[10];
    int tmp_length = 0;

    cpy_array(numbers, tmp_numbers, length);
    tmp_length = length;

    for (int i = 0; i <= length; i++)
    {
        int result = check(tmp_numbers, tmp_length);
        if (result == -1)
            return true;

        printf(" Element to remove: %i \n", numbers[i]);
        cpy_array(numbers, tmp_numbers, length);
        tmp_length = length;
        tmp_length = remove_element(tmp_numbers, tmp_length, i);
    }

    return false;
}

int calculate(const char* line, int length)
{
    int numbers[10]     = {0};
    int numbers_in_line = parse_input(line, length, numbers, 10);

    bool result         = try_all_faulty_indices(numbers, numbers_in_line);

    if (result)
        return 1;

    return 0;
}

int main(int argc, char* argv[])
{
    int accumulated = 0;

    FILE* fp;
    char line[100] = {0};
    size_t len     = 0;
    ssize_t read;

    fp = fopen("../../../day2_input", "r");
    // fp = fopen("../../input_test", "r");

    if (fp == NULL)
        exit(EXIT_FAILURE);

    while (fgets(line, 100, fp))
    {
        int current = calculate(line, 100);
        accumulated += current;
    }
    fclose(fp);

    printf("\nThis is what I calculated: %i\n", accumulated);

    return 0;
}
