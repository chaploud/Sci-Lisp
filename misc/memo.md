# Development Memo

## Keywords & Symbol

### Clojure

```Clojure
()
[]
{}
""
;
+
-
*
/
//
%
`
'
~
~@
->
=
==
<
<=
>
>=
abs
all-ns
and
any?
apply
assert
assoc
assoc!
assoc-in
await
atom
boolean
boolean?
case
char
char?
class
calss?
coll?
comment
comp
concat
cond
conj
conj!
cons
contains?
count
create-ns
cycle
dec
```

### Python

```Python
```

### NumPy

### Matplotlib

### Pandas

### Scipy

### C++

```Cpp
char
int
long
long long
double
float
```

### Sci-Lisp

```lisp
head ; pandas
tail ; pandas
struct ; C++
i8
i16
i32
i64
i32
u8
u16
u32
u64
f32
f64
nan
inf
-inf
-0.0
str
nil
true
false
defn
def
fn
```

#### features

- falsy values are only false and nil
  - 0, '(), [], {}, "": all truthy
- Regexp support
- C-style print format
- csv, json, xml support
- complex number support

#### example

```lisp
(let [a:i32 999
      b:f64 8.6]
  (print a b))
=> nil
999 8.6

(def arr:[i32;3] [1 2 3])
arr
=> [1, 2, 3]

(def vec:v<f64> [2.0, 3.5, 4.5])
vec
=> [2.0, 3.5, 4.5]

(defn double<T>
      [a: T]: T
      (* 2 a))

(double 4.0)
```

## logo

- Round
- Purple based
- (f[x])
