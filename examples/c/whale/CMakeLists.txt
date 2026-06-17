cmake_minimum_required(VERSION 3.10)
project(whale_c)

# Include the header directory
include_directories(../../../c/csrc/include)

# Link the C library directory
link_directories(../../../target/release)

# Create the executable
add_executable(whale_c main.c)

# Link against the C library
target_link_libraries(whale_c longportwhale_c)

# Link ncurses on Unix systems
if(NOT CMAKE_HOST_WIN32)
    target_link_libraries(whale_c ncurses)
endif()
