cmake_minimum_required(VERSION 3.10)
project(whale_cpp)

# Set C++17 standard (required for std::optional)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Include the header directory
include_directories(../../../cpp/include)

# Link the C++ library directory
link_directories(../../../target/release)

# Create the executable
add_executable(whale_cpp main.cpp)

# Link against the C++ library (which wraps the C library)
target_link_libraries(whale_cpp longportwhale_cpp longportwhale_c)
