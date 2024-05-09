include_directories(../../../c/csrc/include)

add_executable(whale_c main.c)
target_link_libraries(whale_c longportwhale-c)
if(NOT CMAKE_HOST_WIN32)
    target_link_libraries(whale_c ncurses)
endif()
