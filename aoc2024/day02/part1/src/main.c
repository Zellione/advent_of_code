#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

int calculate(const char* line, const int length)
{
    char buffer[3]     = {0};
    int buffer_index   = 0;
    int previous_value = -1;
    int direction      = 0;
    bool violation     = false;

    for (int i = 0; i < length; i++)
    {
        if (line[i] != ' ' && line[i] != '\n')
        {
            buffer[buffer_index] = line[i];
            buffer_index++;
        }
        else
        {
            buffer[buffer_index + 1] = '\0';
            int temp                 = atoi(buffer);
            int delta                = 0;
            if (previous_value != -1)
            {
                if (temp > previous_value)
                {
                    if (direction == 0)
                        direction = 1;
                    delta     = temp - previous_value;
                    violation = direction < 0;
                }
                else
                {
                    if (direction == 0)
                        direction = -1;
                    delta     = previous_value - temp;
                    violation = direction > 0;
                }
                if (delta < 1 || delta > 3 || violation)
                {
                    return 0;
                }
            }
            previous_value = temp;
            for (int j = 0; j < buffer_index; j++)
            {
                buffer[j] = 0;
            }
            buffer_index = 0;
        }
    }

    return 1;
}

int main(int argc, char* argv[])
{
    int accumulated = 0;

    FILE* fp;
    char* line;
    size_t len = 0;
    ssize_t read;

    fp = fopen("../../../day2_input", "r");
    // fp = fopen("../../input_test", "r");

    if (fp == NULL)
        exit(EXIT_FAILURE);

    while ((read = getline(&line, &len, fp)) != -1)
    {
        int current = calculate(line, read);
        accumulated += current;
    }
    fclose(fp);

    printf("\nThis is what I calculated: %i\n", accumulated);

    return 0;
}
