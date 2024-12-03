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
        if (line[i] != ' ' && line[i] != '\n')
        {
            buffer[buffer_index] = line[i];
            buffer_index++;
        }
        else
        {
            if (numbers_parsed == length)
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

int check(int* numbers, int length)
{
    for (int i = 0; i < length; i++)
    {
        printf("%i\t", numbers[i]);
    }
    printf("\n");
    int direction = 0;
    for (int i = 0; i < length - 2; i++)
    {
        int delta = numbers[i] - numbers[i + 1];

        if (direction == 0)
            direction = delta > 0 ? 1 : -1;

        int udelta = abs(delta);
        printf("\tDelta: %i Direction: %i Current: %i Previous %i\n", delta, direction, numbers[i + 1], numbers[i]);
        if (udelta > 3 || udelta < 1 || (delta < 0 && direction == 1) || (delta > 0 && direction == -1))
        {
            return i + 1;
        }
    }

    return -1;
}

int remove_element(int* numbers, int length, int element)
{
    int tmp_nums[10];
    for (int i = 0; i < length; i++)
    {
        if (i == element)
            continue;
        tmp_nums[i] = numbers[i];
    }

    for (int i = 0; i < length - 1; i++)
    {
        numbers[i] = tmp_nums[i];
    }

    return length - 1;
}

int calculate(const char* line, int length)
{
    int numbers[10]     = {0};
    int numbers_in_line = parse_input(line, length, numbers, 10);

    // for (int i = 0; i < numbers_in_line; i++)
    // {
    //     printf("%i\t", numbers[i]);
    // }
    printf("\n");
    int index = check(numbers, numbers_in_line);
    if (index == -1)
        return 1;

    // printf("Length: %i\t", numbers_in_line);
    numbers_in_line = remove_element(numbers, numbers_in_line, index);
    // printf("Length: %i\n", numbers_in_line);
    index           = check(numbers, numbers_in_line);
    if (index != -1)
        return 0;

    return 1;
}

int main(int argc, char* argv[])
{
    int accumulated = 0;

    FILE* fp;
    char line[100] = {0};
    size_t len     = 0;
    ssize_t read;

    // fp = fopen("../../../day2_input", "r");
    fp = fopen("../../input_test", "r");

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
