cmake_minimum_required(VERSION 3.14)

project(wordle LANGUAGES CXX)

if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release)
endif()
set(CMAKE_CXX_FLAGS_RELEASE "-O3")
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS 1) # to build compile_commands.json

set(PROJECT_SOURCES
  main.cc

  words.h words.cc
  calc.h calc.cc
)

add_executable(wordle ${PROJECT_SOURCES})
