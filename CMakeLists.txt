cmake_minimum_required(VERSION 3.11)
cmake_policy(SET CMP0048 NEW)
cmake_policy(SET CMP0079 NEW)
project(longport)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS ON)

set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})

include(FetchContent)
fetchcontent_declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.2.1
)
fetchcontent_makeavailable(Corrosion)

add_subdirectory(c)
add_subdirectory(cpp)

add_subdirectory(examples/c/whale)
add_subdirectory(examples/cpp/whale)
