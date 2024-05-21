cmake_minimum_required(VERSION 3.10)
project(RecastNav)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include/)
file(GLOB_RECURSE SOURCE_FILE ${CMAKE_CURRENT_SOURCE_DIR}/source/*.cpp)
add_library(RecastNavLib SHARED ${SOURCE_FILE})
target_link_libraries(RecastNavLib stdc++)