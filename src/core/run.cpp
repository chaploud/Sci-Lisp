#include <string>
#include <iostream>
#include "core.cpp"

using namespace std;

void run(string path) {
  core();
  cout << "run: " << path << endl;
}
