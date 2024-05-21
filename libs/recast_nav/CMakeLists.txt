cmake_minimum_required(VERSION 3.10)
project(RecastNav)

include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include/)
file(GLOB_RECURSE SOURCE_FILE ${CMAKE_CURRENT_SOURCE_DIR}/source/*.cpp)
add_library(RecastNavLib ${SOURCE_FILE})
