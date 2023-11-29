#include <string>
#include <iostream>
#include "core.cpp"

void lint(std::string path, Environment &env) {
  std::cout << "lint: " << path << std::endl;
  std::cout << env << std::endl;
}
