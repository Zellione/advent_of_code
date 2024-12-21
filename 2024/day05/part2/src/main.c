#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define ANSI_COLOR_RED "\x1b[31m"
#define ANSI_COLOR_RESET "\x1b[0m"

// #define TEST

#define PAGES_INDEX_MAX 30
#ifndef TEST
#define INPUT_FILE "../../../day5_input"
#define DEPENDENCY_ORDER_MAX 1200
#else
#define INPUT_FILE "../../input_test"
#define DEPENDENCY_ORDER_MAX 25
#endif

struct DependencyOrderItem
{
    int page_number;
    int requirement;
};

void swap(int* a, int* b)
{
    int tmp = *a;
    *a      = *b;
    *b      = tmp;
}

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

int find_dependency(int page_number, struct DependencyOrderItem** dependency_order_item, int doi_count,
                    int* start_index)
{
    for (int i = *start_index; i < doi_count; i++)
    {
        if (dependency_order_item[i]->page_number == page_number)
        {
            *start_index = i + 1;
            return dependency_order_item[i]->requirement;
        }
    }

    return -1;
}

int get_position(int* pages, int pages_count, int number)
{
    for (int i = 0; i < pages_count; i++)
    {
        if (pages[i] == number)
            return i;
    }

    return -1;
}

int retrieve_middle_page_number(int* pages, int page_count)
{
    return pages[page_count / 2];
}

bool sort_by_dependency_order(int* pages, int pages_count, struct DependencyOrderItem** dependency_order_item,
                              int doi_count)
{
    bool valid       = true;
    bool has_changed = false;
    do
    {
        has_changed = false;
        for (int i = 0; i < pages_count; i++)
        {
            int cur_dependency = 0;
            int dependency     = find_dependency(pages[i], dependency_order_item, doi_count, &cur_dependency);
            int dependency_pos = get_position(pages, pages_count, dependency);
            while (dependency > 0)
            {
                if (i < dependency_pos && dependency_pos != -1)
                {
                    swap(&pages[i], &pages[dependency_pos]);
                    has_changed = true;
                    valid       = false;
                }

                dependency     = find_dependency(pages[i], dependency_order_item, doi_count, &cur_dependency);
                dependency_pos = get_position(pages, pages_count, dependency);
            }
        }
    } while (has_changed);

    return valid;
}

int read_pages(char* line, struct DependencyOrderItem** dependency_order_array, int current_index)
{
    int pages[PAGES_INDEX_MAX] = {0};
    int pages_index            = 0;

    char* token = strtok(line, ",");
    while (token)
    {
        assert(pages_index < PAGES_INDEX_MAX);
        pages[pages_index] = atoi(token);
        token              = strtok(NULL, ",");
        pages_index++;
    }

    bool valid = sort_by_dependency_order(pages, pages_index, dependency_order_array, current_index);

    return valid ? 0 : retrieve_middle_page_number(pages, pages_index);
}

int main(int argc, char* argv[])
{
    uint accumulated = 0;

    FILE* fp;
    char line[1000] = {0};
    ssize_t read;

    struct DependencyOrderItem* dependency_order[DEPENDENCY_ORDER_MAX];
    int dependency_order_count = 0;
    fp                         = fopen(INPUT_FILE, "r");

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
