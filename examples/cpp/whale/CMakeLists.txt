include_directories(../../../cpp/include)

add_executable(whale_cpp main.cpp)
target_link_libraries(whale_cpp longportwhale_cpp)
