cmake_minimum_required(VERSION 3.25)
project(ammonia C)

include(CTest)
enable_testing()

set(CMAKE_C_STANDARD 23)
set(CMAKE_C_STANDARD_REQUIRED ON)

add_library(ammonia_lib
    src/board/bitboard.c
)

target_include_directories(ammonia_lib PUBLIC src)

add_executable(ammonia
    src/main.c
)

target_link_libraries(ammonia PRIVATE ammonia_lib)

file(GLOB TEST_SOURCES CONFIGURE_DEPENDS tests/*.c)

foreach(test_src ${TEST_SOURCES})
    get_filename_component(test_name ${test_src} NAME_WE)

    add_executable(${test_name}
        ${test_src}
    )

    target_link_libraries(${test_name} PRIVATE ammonia_lib)

    add_test(NAME ${test_name} COMMAND ${test_name})
endforeach()
