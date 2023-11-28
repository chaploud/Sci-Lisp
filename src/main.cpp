#include <vector>
#include "core/compile.cpp"
#include "core/repl.cpp"
#include "core/run.cpp"

int main(int argc, const char **argv) {
  std::vector<string> args;
  for (int i = 0; i < argc; i++) {
    args.push_back(argv[i]);
  }

  try {
    if (argc == 1) {
      repl();
    } else if (argc == 2) {
      run(args[1]);
    } else if (argc == 3 && args[1] == "-c") {
      compile(args[2]);
    } else {
      std::cerr << "invalid arguments" << std::endl;
    }
  } catch (std::runtime_error &e) {
    std::cerr << e.what() << std::endl;
  }

  return 0;
}
