#include <string>
#include <iostream>
#include "core.cpp"

void repl(Environment &env) {
  std::string code;
  std::string input;
  Value tmp;
  while (std::cin) {
    std::cout << "λ > ";
    std::getline(std::cin, input);
    if (input == "quit" || input == "q" || input == "exit") {
      break;
    } else if (input == "%env") {
      std::cout << env << std::endl;
    } else if (input != "") {
      try {
        tmp = run(input, env);
        std::cout << "=> " << tmp.debug() << std::endl;
        code += input + "\n";
      } catch (Error &e) {
        std::cerr << e.description() << std::endl;
      } catch (std::runtime_error &e) {
        std::cerr << e.what() << std::endl;
      }
    }
  }
}
