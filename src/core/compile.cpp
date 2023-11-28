#include <string>
#include <iostream>
#include "core.cpp"

using namespace std;

void compile(string path) {
  core();
  cout << "compile: " << path << endl;
}
