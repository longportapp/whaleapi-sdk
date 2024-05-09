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
target_link_libraries(longportwhale_cpp longportwhale-c)
endif()

add_subdirectory(test)
