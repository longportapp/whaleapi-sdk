cmake_minimum_required(VERSION 3.10)
project(longportwhale_cpp)

# Set C++17 standard (required for std::optional)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Link the C library directory
link_directories(../target/release)

include_directories(../c/csrc/include include)
set(SOURCES
    src/http_client.cpp
    src/config.cpp
    src/decimal.cpp
    src/status.cpp
    src/trade_context.cpp
    src/types.cpp
)
add_library(
    longportwhale_cpp SHARED
    ${SOURCES}
)
if(NOT CMAKE_HOST_LINUX)
target_link_libraries(longportwhale_cpp longportwhale_c)
endif()

add_subdirectory(test)
