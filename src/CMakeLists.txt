# cd src
# cmake -S . -B ../build
# cmake --build ../build
cmake_minimum_required(VERSION 3.13)
project(sci-lisp CXX)
add_executable(slisp core/core.cpp)
