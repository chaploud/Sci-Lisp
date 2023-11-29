#include <string>
#include <iostream>
#include "core.cpp"

void compile(std::string path, Environment &env) {
  std::cout << "compile: " << path << std::endl;
  std::cout << env << std::endl;
}
