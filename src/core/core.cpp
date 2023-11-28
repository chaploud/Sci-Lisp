#ifndef _CORE_CPP_
#define _CORE_CPP_

#include <string>
#include <iostream>
#include <sstream>
#include <map>

typedef Value (*Builtin)(std::vector<Value>);

class Value {
 public:

  // Constructors
  Value() : type(NIL) {}
  Value(int64_t i) : type(I64) { stack_data.i = i; }
  Value(double f) : type(F64) { stack_data.f = f; }
  Value(std::vector<Value> list) : type(LIST), list(list) {}

  static Value quote(Value quoted) {
    Value result;
    result.type = QUOTE;
    result.list.push_back(quoted);
    return result;
  }

  static Value atom(std::string s) {
    Value result;
    result.type = ATOM;
    result.str = s;
    return result;
  }

  static Value string(std::string s) {
    Value result;
    result.type = STRING;
    result.str = s;
    return result;
  }

  Value(std::string name, Builtin b) : type(BUILTIN) {
    str = name;
    stack_data.b = b;
  }

  // C+; InterOp Methods
  std::vector<std::string> get_used_atoms() {
    std::vector<std::string> result, tmp;
    switch (type) {
      case QUOTE:
        return list[0].get_used_atoms();
      case ATOM:
        result.push_back(as_atom());
        return result;
      case LIST:
        for (size_t i = 0; i < list.size(); i++) {
          tmp = list[i].get_used_atoms();
          result.insert(result.end(), tmp.begin(), tmp.end());
        }
        return result;
      default:
        result;
    }
  }

  bool is_builtin() { return type == BUILTIN; }

  std::string as_string() const {
    return str;
  }

  std::string as_atom() const {
    return str;
  }

  void push(Value val) {
    list.push_back(val);
  }

  Value pop() {
    Value result = list[list.size() - 1];
    list.pop_back();
    return result;
  }

  // Typecasting Methods
  Value cast_to_i64() const {
    switch (type) {
      case I64:
        return *this;
      case F64:
        return Value(int64_t(stack_data.f));
      default:
        ; // throw Error
    }
  }

  Value cast_to_f64() const {
    switch (type) {
      case F64:
        return *this;
      case I64:
        return Value(double(stack_data.i));
      default:
        ; // throw Error
    }
  }

  // Comparison Operations
  bool operator==(Value other) const {
    if (type == F64 && other.type == I64) {
      return *this == other.cast_to_f64();
    } else if (type == I64 && other.type == F64) {
      return this->cast_to_f64() == other;
    } else if (type != other.type) {
      return false;
    }

    switch (type) {
      case F64:
        return stack_data.f == other.stack_data.f;
      case I64:
        return stack_data.i == other.stack_data.i;
      case BUILTIN:
        return stack_data.b == other.stack_data.b;
      case STRING:
      case ATOM:
        return str == other.str;
      case LIST:
        return list == other.list;
      case QUOTE:
        return list[0] == other.list[0];
      default:
        return true;
    }
  }

  bool operator!=(Value other) const { return !(*this == other); }

  bool operator>=(Value other) const { return !(*this < other); }

  bool operator<=(Value other) const { return (*this == other) || (*this < other); }

  bool operator<(Value other) const {
    if (other.type != F64 && other.type != I64) {
      return false;
    }

    switch (type) {
      case F64:
        return stack_data.f < other.cast_to_f64().stack_data.f;
      case I64:
        if (other.type == F64) {
          return cast_to_f64().stack_data.f < other.stack_data.f;
        } else {
          return stack_data.i < other.stack_data.i;
        }
      default:
        return false;
    }
  }

  Value operator+(Value other) const {
    if (other.type == NIL) {
      return other;
    }

    switch (type) {
      case F64:
        return Value(stack_data.f + other.cast_to_f64().stack_data.f);
      case I64:
        if (other.type == F64) {
          return Value(cast_to_f64() + other.stack_data.f);
        } else {
          return Value(stack_data.i + other.stack_data.i);
        }
      case STRING:
        if (other.type == STRING) {
          return Value::string(str + other.str);
        }
      case LIST:
        if (other.type == LIST) {
          Value result = *this;
          for (size_t i = 0; i < other.list.size(); i++) {
            result.push(other.list[i]);
          }
          return result;
        }
      case NIL:
        return *this;
    }
  }

  std::string display() const {
    std::string result;
    switch (type) {
      case QUOTE:
        return "'" + list[0].debug();
      case ATOM:
        return str;
      case I64:
        return to_string(stack_data.i);
      case F64:
        return to_string(stack_data.f);
      case STRING:
        return str;
      case LIST:
        for (size_t i = 0; i < list.size(); i++) {
          result += list[i].debug();
          if (i < list.size() - 1) {
            result += " ";
          }
        }
        return "(" + result + ")";
      case BUILTIN:
        return "<" + str + " at " + to_string(int64_t(stack_data.b)) + ">";
      case NIL:
        return "nil";
      default:
        // throw Error
        ;
    }
  }

  std::string debug() const {
    std::string result;
    switch (type) {
      case QUOTE:
        return "'" + list[0].debug();
      case ATOM:
        return str;
      case I64:
        return to_string(stack_data.i);
      case F64:
        return to_string(stack_data.f);
      case STRING:
        for (size_t i = 0; i < str.length(); i++) {
          if (str[i] == '"')
            result += "\\\"";
          else
            result.push_back(str[i]);
        }
        return "\"" + result + "\"";
      case LIST:
        for (size_t i = 0; i < list.size(); i++) {
          result += list[i].debug();
          if (i < list.size() - 1) result += " ";
        }
        return "(" + result + ")";
      case BUILTIN:
        return "<" + str + " at " + to_string(long(stack_data.b)) + ">";
      case NIL:
        return "nil";
      default:
        ;
    }
  }

  friend std::ostream &operator<<(std::ostream &os, Value const &v) {
    return os << v.display();
  }

 private:
  enum {
    NIL,
    I64,
    F64,
    LIST,
    QUOTE,
    ATOM,
    STRING,
    BUILTIN
  } type;

  union {
    int64_t i;
    double f;
    Builtin b;
  } stack_data;

  std::string str;
  std::vector<Value> list;
};

void skip_whitespace(std::string &s, int &ptr) {
  while (isspace(s[ptr])) {
    ptr++;
  }
}

Value parse(std::string &s, int &ptr) {
  skip_whitespace(s, ptr);

  // Skip comments
  while (s[ptr] == ';') {
    // If this is a comment
    int work_ptr = ptr;
    // Skip to the end of the line
    while (s[work_ptr] != '\n' && work_ptr < int(s.length())) {
      work_ptr++;
    }
    ptr = work_ptr;
    skip_whitespace(s, ptr);

    // If we're at the end of the string, return an empty value
    if (s.substr(ptr, s.length() - ptr - 1) == "") return Value();
  }

  // Parse the value
  if (s == "") {
    return Value();
  } else if (s[ptr] == '\'') {
    // If this is a quote
    ptr++;
    return Value::quote(parse(s, ptr));

  } else if (s[ptr] == '(') {
    // If this is a list
    skip_whitespace(s, ++ptr);

    Value result = Value(std::vector<Value>());

    while (s[ptr] != ')') result.push(parse(s, ptr));

    skip_whitespace(s, ++ptr);
    return result;

  } else if (isdigit(s[ptr]) || (s[ptr] == '-' && isdigit(s[ptr + 1]))) {
    // If this is a number
    bool negate = s[ptr] == '-';
    if (negate) ptr++;

    int save_ptr = ptr;
    while (isdigit(s[ptr]) || s[ptr] == '.') ptr++;
    std::string n = s.substr(save_ptr, ptr);
    skip_whitespace(s, ptr);

    if (n.find('.') != std::string::npos)
      return Value((negate ? -1 : 1) * atof(n.c_str()));
    else
      return Value((negate ? -1 : 1) * atoi(n.c_str()));

  } else if (s[ptr] == '\"') {
    // If this is a string
    int n = 1;
    while (s[ptr + n] != '\"') {
      if (ptr + n >= int(s.length())) throw std::runtime_error(MALFORMED_PROGRAM);

      if (s[ptr + n] == '\\') n++;
      n++;
    }

    std::string x = s.substr(ptr + 1, n - 1);
    ptr += n + 1;
    skip_whitespace(s, ptr);

    // Iterate over the characters in the string, and
    // replace escaped characters with their intended values.
    for (size_t i = 0; i < x.size(); i++) {
      if (x[i] == '\\' && x[i + 1] == '\\')
        x.replace(i, 2, "\\");
      else if (x[i] == '\\' && x[i + 1] == '"')
        x.replace(i, 2, "\"");
      else if (x[i] == '\\' && x[i + 1] == 'n')
        x.replace(i, 2, "\n");
      else if (x[i] == '\\' && x[i + 1] == 't')
        x.replace(i, 2, "\t");
    }

    return Value::string(x);
  } else if (s[ptr] == '@') {
    ptr++;
    skip_whitespace(s, ptr);
    return Value();

  } else if (is_symbol(s[ptr])) {
    // If this is a string
    int n = 0;
    while (is_symbol(s[ptr + n])) {
      n++;
    }

    std::string x = s.substr(ptr, n);
    ptr += n;
    skip_whitespace(s, ptr);
    return Value::atom(x);
  } else {
    throw std::runtime_error(MALFORMED_PROGRAM);
  }
}

// Parse an entire program and get its list of expressions.
std::vector<Value> parse(std::string s) {
  int i = 0, last_i = -1;
  std::vector<Value> result;
  // While the parser is making progress (while the pointer is moving right)
  // and the pointer hasn't reached the end of the string,
  while (last_i != i && i <= int(s.length() - 1)) {
    // Parse another expression and add it to the list.
    last_i = i;
    result.push_back(parse(s, i));
  }

  // Return the list of values parsed.
  return result;
}


std::string run(string path_or_code, bool is_path = false) {
  std::stringstream ss;
  if (is_path) {
    ss << "run: " << path_or_code;
  } else {
    ss << "run: lisp code";
  }

  std::string result = ss.str();
  std::cout << result << std::endl;

  return result;
}

#endif
