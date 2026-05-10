cmake_minimum_required(VERSION 3.10)
project(perplexity_truth_engine CXX)

set(CMAKE_CXX_STANDARD 17)

add_library(perplexity_truth_engine STATIC engine.cpp)
target_include_directories(perplexity_truth_engine PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
