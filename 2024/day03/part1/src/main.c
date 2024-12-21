#include <assert.h>
#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void clean_buffer(char* buffer, int buffer_length)
{
    for (int i = 0; i < buffer_length; i++)
    {
        buffer[i] = 0;
    }
}

void str_copy(char* buffer, int pos)
{
    char temp[10000] = {0};
    int index        = 0;
    int tmp_pos      = pos;

    while (buffer[tmp_pos] != '\0')
    {
        temp[index] = buffer[tmp_pos];
        index++;
        tmp_pos++;
    }
    temp[index] = '\0';

    for (int i = 0; i <= index; i++)
    {
        buffer[i] = temp[i];
    }
}

int main(int argc, char* argv[])
{
    uint accumulated = 0;

    FILE* fp;
    char line[10000] = {0};
    size_t len       = 0;
    ssize_t read;

    fp = fopen("../../../day3_input", "r");
    // fp = fopen("../../input_test", "r");

    if (fp == NULL)
        exit(EXIT_FAILURE);

    char* pos;
    char buffer[3]  = {0};
    int buffer_pos  = 0;
    int current_pos = -1;
    while (fgets(line, 10000, fp))
    {
        while (true)
        {
            int first_num  = 0;
            int second_num = 0;
            pos            = strstr(line, "mul(");
            if (pos == NULL)
                break;
            if (pos != NULL)
            {
                pos = pos + 4; // this pos should be the first num
                while (isdigit(*pos) || *pos == ',')
                {
                    if (*pos == ',')
                    {
                        first_num = atoi(buffer);
                        printf("\n%i,", first_num);
                        clean_buffer(buffer, 3);
                        buffer_pos = 0;
                    }

                    if (isdigit(*pos))
                    {
                        buffer[buffer_pos] = *pos;
                        buffer_pos++;
                    }

                    pos++;
                }

                if (*pos == ')')
                {
                    second_num = atoi(buffer);
                    printf("%i\n", second_num);
                }
                buffer_pos = 0;
                clean_buffer(buffer, 3);

                if (first_num > 0 && second_num > 0)
                {
                    accumulated += first_num * second_num;
                }

                first_num = second_num = 0;
                current_pos            = (int)(pos - line);
                str_copy(line, current_pos);
            }
        }
    }
    fclose(fp);

    printf("\nThis is what I calculated: %i\n", accumulated);

    return 0;
}
