#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define ANSI_COLOR_RED "\x1b[31m"
#define ANSI_COLOR_RESET "\x1b[0m"

#define TEST

#define WORD_LENGTH 3
#ifndef TEST
#define INPUT_FILE "../../../day4_input"
#define DEPDENCY_ORDER_MAX 1200
#else
#define INPUT_FILE "../../input_test"
#define DEPENDENCY_ORDER_MAX 25
#endif

struct DependencyOrderItem
{
    int page_number;
    int requirement;
};

void read_dependency(char* line, struct DependencyOrderItem** dependency_order_array, int* current_index)
{
    char* token = strtok(line, "|");
    bool first  = true;

    struct DependencyOrderItem* dependency_order_item =
        (struct DependencyOrderItem*)malloc(sizeof(struct DependencyOrderItem));

    while (token)
    {
        if (first)
            dependency_order_item->requirement = atoi(token);
        else
            dependency_order_item->page_number = atoi(token);
        dependency_order_array[*current_index] = dependency_order_item;
        token                                  = strtok(NULL, "|");
        first                                  = false;
    }
    *current_index += 1;
}

int read_pages(char* line, struct DependencyOrderItem** dependency_order_array, int current_index)
{
    // TODO: move this dependency order check into a seperate function
    for (int i = 0; i < current_index; i++)
    {
        printf("%i %i\n", dependency_order_array[i]->page_number, dependency_order_array[i]->requirement);
    }

    return 0;
}

int main(int argc, char* argv[])
{
    uint accumulated = 0;

    FILE* fp;
    char line[1000] = {0};
    ssize_t read;

    struct DependencyOrderItem* dependency_order[DEPENDENCY_ORDER_MAX];
    int dependency_order_count = 0;

    fp = fopen(INPUT_FILE, "r");

    if (fp == NULL)
        exit(EXIT_FAILURE);

    while (fgets(line, 1000, fp))
    {
        if (strstr(line, "|"))
            read_dependency(line, dependency_order, &dependency_order_count);
        if (strstr(line, ","))
            accumulated += read_pages(line, dependency_order, dependency_order_count);
    }
    fclose(fp);

    for (int i = 0; i < dependency_order_count; i++)
        free(dependency_order[i]);

    printf("\nThis is what I calculated: %i\n", accumulated);

    return 0;
}
