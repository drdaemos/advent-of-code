cmake_minimum_required(VERSION 3.20)
project(advent_of_code)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Dependencies via vcpkg
find_package(fmt CONFIG REQUIRED)
find_package(argparse CONFIG REQUIRED)

# Automatically include all day_*.cpp files from twenty_five directory
file(GLOB TWENTY_FIVE_SOURCES "src/twenty_five/day_*.cpp")
file(GLOB SHARED_SOURCES "src/shared/*.cpp")

add_executable(advent_of_code
    src/main.cpp
    ${TWENTY_FIVE_SOURCES}
    ${SHARED_SOURCES}
)
target_include_directories(advent_of_code PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/src)
target_link_libraries(advent_of_code PRIVATE fmt::fmt)
target_link_libraries(advent_of_code PRIVATE argparse::argparse)

# Testing
enable_testing()
find_package(Catch2 3 CONFIG REQUIRED)

# Create a library with all the day solutions (so we can link against it in tests)
add_library(aoc_solutions
    ${TWENTY_FIVE_SOURCES}
    ${SHARED_SOURCES}
)
target_include_directories(aoc_solutions PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/src)
target_link_libraries(aoc_solutions PUBLIC fmt::fmt)

# Test executable
file(GLOB TEST_SOURCES "tests/*.cpp")
add_executable(aoc_tests ${TEST_SOURCES})
target_link_libraries(aoc_tests PRIVATE aoc_solutions Catch2::Catch2WithMain)
target_include_directories(aoc_tests PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/src)

# Register tests with CTest
include(CTest)
include(Catch)
catch_discover_tests(aoc_tests)

find_program(CLANG_FORMAT clang-format)
if(CLANG_FORMAT)
    file(GLOB_RECURSE ALL_CXX_SOURCE_FILES
        ${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp
        ${CMAKE_CURRENT_SOURCE_DIR}/src/*.h
    )
    add_custom_target(format
        COMMAND ${CLANG_FORMAT} -i ${ALL_CXX_SOURCE_FILES}
        COMMENT "Running clang-format"
    )
endif()