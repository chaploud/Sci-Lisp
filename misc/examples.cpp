#include <cmath>
#include <iostream>
#include <limits>

#define STR "str"
#define REGEX "regex"
#define BOOL "bool"
#define NIL "nil"
#define I64 "i64"
#define F64 "f64"
#define SYM "sym"
#define LIST "list"

template <typename T>
class Object {
 public:
  Object() {}
  friend std::ostream& operator<<(std::ostream& os, const Object& v) { return os << v.display(); }
  friend std::string type(const Object& v) { return v.m_type; }

 protected:
  T m_value;
  std::string m_type;
  virtual std::string display() const { return "object"; }
  std::string type() const { return m_type; }
};

class str : public Object<std::string> {
 public:
  str(const char* value) {
    m_value = std::string(value);
    m_type = STR;
  }
 private:
   std::string display() const override {
    return m_value;
   }
};

template <class T>
void type(const Object<T>& obj) {
  return obj.type();
}


using str_t = std::string;    // str
using i64_t = long long;      // i64
using f64_t = double;         // f64
using bool_t = bool;          // bool

// nil
struct nil_t {};
constexpr nil_t nil;

// nan
constexpr f64_t NaN = std::numeric_limits<f64_t>::quiet_NaN();
// inf
constexpr f64_t inf = std::numeric_limits<f64_t>::infinity();
// -inf
constexpr f64_t ninf = -inf;

// symbol
struct sym_t {
  str_t name;
};

std::ostream& operator<<(std::ostream& os, const nil_t& x) {
  os << "nil";
  return os;
}

std::ostream& operator<<(std::ostream& os, const f64_t& x) {
  if (x == NaN) {
    os << "nan";
  } else if (x == inf) {
    os << "inf";
  } else if (x == ninf) {
    os << "-inf";
  } else if (x == 0) {
    if (std::signbit(x)) {
      os << "0.0";
    } else {
      os << "-0.0";
    }
  }
  return os;
}

std::ostream& operator<<(std::ostream& os, const sym_t& x) {
  os << ":" << x.name.c_str();
  return os;
}

void init(void) { std::cout << std::boolalpha; }

int main(void) {
  init();

  str s = "hoge";
  std::cout << s << std::endl;
  std::cout << type(s) << std::endl;



  return 0;
}
