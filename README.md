# (Sci-Lisp)

A Lisp for Scientific Computing written in C++. (Now in progress).

## Features(Plan)

Grand Ambitions. Maybe there are some inconsistencies.

- [ ] S-expression
- [ ] REPL
- [ ] Run as script
- [ ] Compile to single binary
- [ ] Easy installation (Cross Platform)
- [ ] Well Documented
- [ ] Great developer experience with VSCode extension
- [ ] Multiparadigm (Functional, Object-Oriented, Procedural)
- [ ] Typed. And powerful type inference like TypeScript/Rust
- [ ] Seamless use of standard C++ Libraries
- [ ] Can use Clojure-like syntax (), [], {}, etc...
- [ ] Numpy-like array slice ([2:-1, -3:] array2d)
- [ ] Standard with IPython, numpy, matplotlib, pandas, and scipy functionality
- [ ] Signal Processing, Time Series Analysis included as standard
- [ ] Supports exception handling

## Motivation

- Understanding C++ specifications
- Understanding of numpy, matplotlib, pandas, and scipy specifications
- Get domain knowledge of signal processing and time series analysis
- Improve development experience by developing VSCode extensions
- Participation in OSS activities on GitHub

## Influenced by

- Common Lisp
- Clojure
- Python
- Hy(https://github.com/hylang/hy)
- C++
- R
- TypeScript
- Rust
- SQL
- wisp(https://github.com/adam-mcdaniel/wisp)

## Philosophy

Let's use Lisp easily without thinking too hard.

## What we want to focus on

- Readability
- Rich REPL
- High Performance
- Code Simplicity
- Allow for loop
- Allow assignment
- Minimum Keywords
- interpreter, compiler and linter all in one binary

...Some may think it's dirty Lisp...

## Prerequests

### Linux

- g++
- CMake

### Windows

Unsupported yet.

### Mac

Unsupported yet.

## Installation

```bash
# Clone Repository
git clone https://github.com/chaploud/Sci-Lisp.git

# build scilisp binary using CMake
cd Sci-Lisp
./build.sh  # => biuld/scilisp
```

## Command Options

```bash
scilisp  # launch REPL
scilisp xxx.lisp  # run as script
scilisp -c xxx.lisp  # compile code
scllisp -l xxx.lisp  #lint code
```

## To Me

- Small Start!
- Make interpreter first.
