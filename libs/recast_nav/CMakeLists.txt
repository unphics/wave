cmake_minimum_required(VERSION 3.10)
project(RecastNav)

# add_subdirectory(Detour)
# add_subdirectory(Recast)

# Detour
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/Detour/Include/)
# cmake的路径大小写敏感, 记得recast和detour的包含路径是大写的"Include"
link_libraries(libDetour)
link_directories(${CMAKE_CURRENT_SOURCE_DIR}/Detour/build/)

#Recast
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/Recast/Include/)
link_libraries(libRecast)
link_directories(${CMAKE_CURRENT_SOURCE_DIR}/Recast/build/)

include_directories(${CMAKE_CURRENT_SOURCE_DIR}/RecastNav/include/)
file(GLOB_RECURSE SOURCE_FILE ${CMAKE_CURRENT_SOURCE_DIR}/RecastNav/source/*.cpp)
add_library(RecastNavLib ${SOURCE_FILE})

target_link_libraries(RecastNavLib libDetour libRecast)