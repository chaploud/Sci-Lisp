#include <cmath>
#include <iostream>
#include <limits>
#include <regex>
#include <string>
#include <iomanip>

#define STR "str"
#define REGEX "regex"
#define BOOL "bool"
#define NIL "nil"
#define I64 "i64"
#define F64 "f64"
#define SYM "sym"
#define LIST "list"

// base class
template <typename T>
class Object {
 public:
  friend std::ostream& operator<<(std::ostream& os, const Object& v) { return os << v.display(); }
  friend std::string type(const Object& v) { return v.m_type; }

 protected:
  T m_value;
  std::string m_type;
  virtual std::string display() const { return "object"; }
  std::string type() const { return m_type; }
};

// str
class str : public Object<std::string> {
 public:
  str(const char* value) {
    m_value = std::string(value);
    m_type = STR;
  }
  str& operator=(const str& x) {
    m_value = x.m_value;
    return *this;
  }
  str& operator=(const std::string& x) {
    m_value = x;
    return *this;
  }
 private:
   std::string display() const override {
    return "\"" + m_value + "\"";
   }
};

// regex
class regex : public Object<std::regex> {
 public:
  regex(const char* value) {
    m_string = std::string(value);
    m_value = std::regex(m_string);
    m_type = REGEX;
  }
 private:
  std::string m_string;
  std::string display() const override {
    return "regex(\"" + m_string + "\")";
  }
};

// bool
// needs call function init() to display bool

// nil
struct nil_t {}; // TODO: compare

class nil : public Object<nil_t> {
 public:
  nil() {
    m_value = {};
    m_type = NIL;
  }
 private:
  std::string display() const override {
    return "nil";
  }
};

// i64
using i64 = long long;

// f64
using f64 = double;

// symbol
struct sym_t {};

// sym
class sym : public Object<sym_t> {
 public:
  sym(const char* value) {
    m_string = std::string(value);
    m_value = {}; // TODO: compare
    m_type = SYM;
  }
 private:
  std::string m_string;
  std::string display() const override {
    return m_string;
  }
};

template <class T>
std::string type(const Object<T>& obj) {
  return obj.type();
}

// for bool
std::string type(bool obj) {
  return BOOL;
}
// for i64
std::string type(i64 obj) {
  return I64;
}
// for f64
std::string type(f64 obj) {
  return F64;
}

// nan
const f64 NaN = std::numeric_limits<f64>::quiet_NaN();
// inf
const f64 inf = std::numeric_limits<f64>::infinity();
// -inf
const f64 ninf = -inf;

void init(void) {
  std::cout << std::boolalpha;
  std::cout << std::scientific << std::setprecision(5);
}

int main(void) {
  init();

  str s = "hoge";
  std::cout << s << " " << type(s) << std::endl;

  regex re = R"([0-9]+\.\d*)";
  std::cout << re << " " << type(re) << std::endl;

  bool tr = true;
  bool fa = false;
  std::cout << tr << " " << fa << " " << type(tr) << std::endl;

  nil ni;
  std::cout << ni << " " << type(ni) << std::endl;

  i64 i = -999;
  std::cout << i << " " << type(i) << std::endl;

  f64 f = -3.141592;
  std::cout << f << " " << type(f) << std::endl;

  f64 fe = -3.14e15;
  std::cout << fe << " " << type(fe) << std::endl;

  f64 na = NaN;
  f64 posinf = inf;
  f64 neginf = ninf;
  std::cout << na << " ";
  std::cout << posinf << " ";
  std::cout << neginf << std::endl;

  sym sy = sym(":hoge");
  std::cout << sy << " " << type(sy) << std::endl;

  return 0;
}
