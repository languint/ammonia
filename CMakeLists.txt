cmake_minimum_required(VERSION 4.1.2)
project(ammonia)

add_executable(ammonia)

target_sources(
  ammonia
  PRIVATE src/main.c
)
