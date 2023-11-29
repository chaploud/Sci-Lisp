#include <vector>
#include "core/core.cpp"
#include "core/compile.cpp"
#include "core/repl.cpp"
#include "core/lint.cpp"

int main(int argc, const char **argv) {
  Environment env;
  std::vector<string> args;
  for (int i = 0; i < argc; i++) {
    args.push_back(argv[i]);
  }
  env.set("cmd-args", Value(args));

  srand(time(NULL));

  try {
    if (argc == 1) {
      repl(env);
    } else if (argc == 2) {
      run(read_file_contents(argv[1]), env);
    } else if (argc == 3 && args[1] == "-c") {
      compile(args[2], env);
    } else if (argc == 3 && args[1] == "-l") {
      lint(args[2], env);
    } else {
      std::cerr << "invalid arguments" << std::endl;
    }
  } catch (Error &e) {
    std::cerr << e.description() << std::endl;
  } catch (std::runtime_error &e) {
    std::cerr << e.what() << std::endl;
  }

  return 0;
}
