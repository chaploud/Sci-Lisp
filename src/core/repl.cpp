#include <string>
#include <iostream>
#include "core.cpp"

using namespace std;

void repl() {
  string input;
  string result;

  while (cin) {
    cout << "λ > ";
    getline(cin, input);
    if (input == "quit" || input == "q" || input == "exit") {
      break;
    } else if (input != "") {
      try {
        result = run(input, false);
        cout << "=> " << result << endl;
      } catch (runtime_error &e) {
        cerr << e.what() << endl;
      }
    }
  }
}
