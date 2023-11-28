#ifndef _RUN_CPP_
#define _RUN_CPP_

#include <string>
#include <iostream>
#include <sstream>
#include "core.cpp"

using namespace std;

string run(string path_or_code, bool is_path = false) {
  core();
  stringstream ss;
  if (is_path) {
    ss << "run: " << path_or_code;
  } else {
    ss << "run: lisp code";
  }

  string result = ss.str();
  cout << result << endl;

  return result;
}

#endif
