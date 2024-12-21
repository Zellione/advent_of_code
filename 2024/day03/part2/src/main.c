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

int find_do_or_dont(char* line, char* pos)
{
    char* pos_do   = NULL;
    char* pos_dont = NULL;

    pos_do   = strstr(line, "do()");
    pos_dont = strstr(line, "don't()");

    if (pos_do < pos || pos_dont < pos)
    {
        if (pos_do < pos && pos_do != NULL)
            return 1;

        if (pos_dont < pos && pos_dont != NULL)
            return 0;
    }

    return -1;
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

    char* pos_mul;
    char buffer[3]       = {0};
    int buffer_pos       = 0;
    int current_pos      = -1;
    bool allowed         = true;
    int found_do_or_dont = -1;
    while (fgets(line, 10000, fp))
    {
        while (true)
        {
            int first_num  = 0;
            int second_num = 0;
            pos_mul        = strstr(line, "mul(");
            if (pos_mul == NULL)
                break;
            if (pos_mul != NULL)
            {
                found_do_or_dont = find_do_or_dont(line, pos_mul);
                if (found_do_or_dont == 1)
                    allowed = true;
                else if (found_do_or_dont == 0)
                    allowed = false;

                pos_mul = pos_mul + 4; // this pos should be the first num
                while (isdigit(*pos_mul) || *pos_mul == ',')
                {
                    if (*pos_mul == ',')
                    {
                        first_num = atoi(buffer);
                        clean_buffer(buffer, 3);
                        buffer_pos = 0;
                    }

                    if (isdigit(*pos_mul))
                    {
                        buffer[buffer_pos] = *pos_mul;
                        buffer_pos++;
                    }

                    pos_mul++;
                }

                if (*pos_mul == ')' && allowed)
                {
                    second_num = atoi(buffer);
                    printf("\n%i,%i\n", first_num, second_num);
                }
                buffer_pos = 0;
                clean_buffer(buffer, 3);

                if (first_num > 0 && second_num > 0 && allowed)
                {
                    accumulated += first_num * second_num;
                }

                first_num = second_num = 0;
                current_pos            = (int)(pos_mul - line);
                str_copy(line, current_pos);
            }
        }
    }
    fclose(fp);

    printf("\nThis is what I calculated: %i\n", accumulated);

    return 0;
}
