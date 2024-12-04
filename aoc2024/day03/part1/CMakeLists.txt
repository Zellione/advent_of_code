cmake_minimum_required(VERSION 3.31)

project (AdventofCode)

set (CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Werror")
set (source_dir "${PROJECT_SOURCE_DIR}/src")
message(STATUS "${PROJECT_SOURCE_DIR}")

file (GLOB_RECURSE source_files "${source_dir}/*.c")

message(STATUS "source_files=${source_files}")

add_executable (advent_of_code_day3_part1 ${source_files})

target_link_libraries(advent_of_code_day3_part1)
