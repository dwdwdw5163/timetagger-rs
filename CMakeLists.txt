cmake_minimum_required(VERSION 3.28)

project(test)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Common configuration for all targets
set(COMMON_INCLUDE_DIRS /usr/include/timetagger)
set(COMMON_LIBRARIES TimeTagger)
set(COMMON_COMPILE_OPTIONS -MD -O2 -DNDEBUG)

add_executable(test src/test.cpp src/timetagger.cpp)
target_include_directories(test PUBLIC ${COMMON_INCLUDE_DIRS})
target_link_libraries(test PUBLIC ${COMMON_LIBRARIES})
target_compile_options(test PUBLIC ${COMMON_COMPILE_OPTIONS})