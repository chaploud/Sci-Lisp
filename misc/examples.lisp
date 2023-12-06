; Comment

;; Literal
'()              ; list
`()              ; quoted List (TODO:)
[]               ; vector
{}               ; hash map
#{}              ; hash set
""               ; string
"\n"             ; escape sequence
#"[0-9]+"        ; regular expression
false            ; false
true             ; true
nil              ; nil (means null/None) (TODO:)
0x1a             ; hexadecimal (TODO:)
0o17             ; octal (TODO:)
0b0101           ; binary (TODO:)
1+2j             ; complex number (TODO:)
-3.14e15         ; float
-999             ; int
nan              ; Not A Number
inf              ; positive infinity
-inf             ; negative infinity
-0.0             ; negative zero
:symbol          ; symbol

;; Keywords
(ns) ; (TODO:)
(import) ; (TODO:)
(let)
(const)
(def)
(defn)
(fn)
(set!)
(return)
(yield)
(if)
(when)
(cond)
(switch)
(do)
(for)
(while)
(break)
(continue)
(struct)
(enum)
(class)
(this)
(macro)
(throw)
(try)
(catch)

;; Built-ins

;; Arithmetic
(inc 1)              ; add 1
(dec 1)              ; subtract 1
(+ 1 1)              ; Add
(- 1 1)              ; Subtract
(* 2 3)              ; product
(/ 1 2)              ; devide (float)
(// 1 2)             ; devide
(% 3 2)              ; remainder

;; Compare
(= 2 2)              ; equal
(is [1, 2] [1, 2])   ; ident (=> false)
(< 2 3)              ; less
(<= 2 3)             ; less equal
(> 2 3)              ; greater
(>= 2 3)             ; greater equal

;; Logical
(and true false)     ; and
(or true false)      ; or
(xor true true)      ; xor
(not false)          ; not

;; Math
(abs -2)             ; absolute value
(abs [-2, 3, 4])     ; absolute value applying to vector
(cos 2.0)            ; cosine
(sin 2.0)            ; sine
(tan 2.0)            ; tangent
(acos 2.0)           ; arccosine
(asin 2.0)           ; arcsine
(atan 2.0)           ; arctangent
(log 2.0)            ; log_e
(log10 2.0)          ; log_10
(rand)               ; rondom value 0.0 to 1.0
(randint 30)         ; random integer 0 to n

;; Utility
(type [1, 2, 3])     ; show type
(time (+ 1 2))       ; measure processing time
(print {:a 2, :b 3}) ; print any
(printf "{0:02d}kg" 56) ; print format

;; Definition
(let [a 2]           ; bind variable
  (set! a 3)         ; assign(destructive)
  (print a))         ; print

(def a 2)            ; variable
(const C [1, 2, 3])  ; constant value (can't assgin after)

(defn func [a,       ; function
            b 32]    ; default argument
  (print a)
  (+ a b))
(func 2 3)           ; positional arguments
(func {:a 2, :b 3})  ; named arguments
(func 2 {:b 3})      ; mixed above
(def lst '(4, 5))
(func @lst)          ; extract arguments

;; Control Flow
(if (< 2 3)          ; if condition
  "true"
  "false")

(when (< 2 3)        ; when codition
  (do                ; do multiple expressions
    (print "2 < 3")
    "retval"))

(cond                ; cond condition
  (< n 0) "negative"
  (> n 0) "positive"
  :else "default")

(def val "hoge")
(switch val          ; switch condition
  ["a"]
    (print "A")
  ["b", "c"]
    (print "B or C")
  :default
    (print "DEFAULT"))

(for [i (range 5)]   ; for loop, range
  (print i))

(def a 0)
(while (< a 100)     ; while loop
  (print a)
  (set! a (+ a 1))   ; add and assign
  (if (> a 50)
     (break)         ; break
     (continue)))    ; continue

;; Cast
(bool 2)                 ; to bool
(int 2.0)                ; to int
(float -3)               ; to float
(str :a)                 ; to string
(sym "a")                ; to symbol

;; Collection
(list 1 2 3)             ; make list
(vector '(1 2 3))        ; coerce vector
(hmap '(:a 1 :b 2))      ; coerce hash map
(hset '(:a :b))          ; coerce hash set

;; Predicate
(nil? false)             ; is nil?
(true? nil)              ; is true?
(false? true)            ; is false?
(bool? 2)                ; is bool?
(int? 2.3)               ; is int?
(float? "a")
(str? :a)
(sym? 3)
(list? '(1 2 3))
(vector? [1 2 3])
(hmap? {:a 1, :b 2})
(hset? #{:a, :b})
(empty? "")
(zero? 9)
(nan? inf)
(inf? -inf)
(-inf? nan)
(in? 1 [3, 2, 1])

;; String
(format "{0:03d} kg" 56)      ; format string
(join "," ["1", "2", "3"])    ; join (=> "1,2,3")
(in? "a" "12aabc32")          ; is string in string?
(upper "abc")                 ; upper-case
(lower "DEF")                 ; lower-case

;; RegExp
(find #"[0-9]+" "aa123a")             ; => "123"
(match #"hello, (.*)" "hello, world") ; => ["hello, world", "world"]

;; Vector
(shape [[1, 2], [3, 4], [5, 6]]) ; shape of vector (=> [3, 2])
(len [1, 2, 3])                  ; length of vector
(sum [1, 2, 3])                  ; sum of vector
(mean [1, 2, 3])                 ; mean of vector
(max [1, 2, 3])                  ; max of vector
(min [1, 2, 3])                  ; min of vector
(std [1, 2, 3])                  ; standard deviation of vector
(head [1, 2, 3] 2)
(tail [1, 2, 3] 2)
(cycle)
(chunk)
(replace)
(reverse)
(repeat)
(some? [false, true, false])
(every? [false, true, false])

(sort [3, 1, 2])                 ; sort (non-destructive)
(shuffle [3, 1, 2])              ; shuffle (non-destructive)
(push [3, 1, 2] 4)               ; push_back (non-destructive)
(cons [3, 1, 2] 4)               ; push_front (non-destructive)

(def v [3, 1, 2])
(sort! v)                        ; sort (destructive)
(shuffle! v)                     ; shuffle (destructive)
(push! v 4)                      ; push_back (destructive)
(cons! v 4)                      ; push_front (destructive)

;; hashmap
(assoc)
(assoc!)

;; slicing
([1] [[1, 2], [3, 4], [5, 6]])         ; => [3, 4]
([0:2] [[1, 2], [3, 4], [5, 6]])       ; => [[1, 2], [3, 4]]
([0:-1, 1] [[1, 2], [3, 4], [5, 6]])   ; => [2, 4]
([1, 1] [[1, 2], [3, 4], [5, 6]])      ; => 4
([:a] {:a 2, :b 3})                    ; => 2

;; functional programming
(filter)
(map)
(apply)
(comp)

;; I/O
;; TODO: Need with open
;; TODO: Stream
(read)
(write)

;; struct/enum/class/macro
(struct)
(enum)
(class)
(macro)
;; throw/try/catch
(throw)
(try)
(catch)
;; namespace
(ns)


;; Memo
;; === Keywords
;; Must
false
true
nil
when
cond
import  ;; Don't confuse users
def
defn
fn
for
while
range
break
continue
try
catch
throw
return
and
or
xor
not
del
is
+
+=
++
-
-=
--
*
*=
/
/=
//
//=
>
>=
<
<=
=  ; assgin/set
== ; compare
apply
ns
assoc
assoc!
assoc-in
atom
assert
head
tail
sym-to-str
str-to-sym
chunk
class?
string?
int?(i8?, i16?, ...)
float?(f32?, f64?)
coll?
comment
comp
cond
conj
conj!
cons
contains?(in?)
count(len/size)
cycle
struct
declare
method
let
format
const
dissoc
doseq
empty?
even?
odd?
enumerate
push
pop
first
last
false?
true?
filter
map
find
flatten
zip
fn?
group-by
if-let
juxt
into
keys
values
list
macroexpand
macroexpand-1
max
min
log
log10
sin
cos
tan
atan
acos
merge
mod
nan?
neg?
pos?
new
print
printf
nil?
inf?
-inf?
any?
some?
every?
not-any?
not-empty?
not-every?
partial
quot
rem
quote
rand
rand-int
read
write
remove
repeat
replace
rest
reverse
second
seq
seq?
shuffle
sort
sort-by
str
swap!
sym
sym?
time
type
update
with-open
zero?
enum
private
protected
public
defmacro
`
~
~@
0x2e(hex)
0o27(octal)
0b1011(binary)
hset ;; coerce
hmap
vector
list

;; Want
async
await
in (contain)
with
pass
as
yield
switch, case

;; Choice

;; Functions
vector
list
string

;; Scientific/Math
sum
mean
std
count/size/len
abs
cos
sin
tan

;; Types (Literal)
fn           ; function type
any          ; any type
nil          ; nil
sym          ; symbol :blur
bool         ; true/false
complex      ; 1 + 5.2j
i8           ; 100i8    i8 is optional
i16          ; 100i16   i16 is optional
i32          ; 100i32   i32 is optional
i64          ; 100i64   i64 is optional
i128         ; 100i128  i128 is optional
u8           ; 100u8    u8 is optional
u16          ; 100u16   u16 is optional
u32          ; 100u32   u32 is optional
u64          ; 100u64   u64 is optional
u128         ; 100u128  u128 is optional
f32          ; 8.6f32   f32 is optional
f64          ; 8.6f64   f64 is optional
ibig         ; 10000ibig arbitrary precision integer(can use N literal)
fbig         ; 3.141fbig arbitrary precision float(can use M literal)
str          ; string  "lisp"
regex        ; Regular Expression #"[0-9]+"
list         ; linked list '(1, 2, 3) allow any type combination
v[T][i,j,k]  ; vector [2.0, 3.0, 4.0]:v[f64][3]
m[K,V]       ; HashMap {:a 1, :b 2, :c 3}:m[sym,i32]
s[K]         ; HashSet #{"a", "b", "c"}:s[str]
om[K,V]      ; Ordered HashMap {:a 1, :b 2, :c 3}:om[sym,i32]
os[K]        ; Ordered HashSet #{"a", "b", "c"}:os[str]

;; === Syntax
(def a :bool true)  ;; (def a true) is OK (Type inference)
(def a :sym :blur)  ;; TODO: lexical analysis
(def a :i32 36)     ;; (def a 36) => a:i64 selected
(def a :f64 3.1415) ;; (def a 3.14) => a:f64 selcted
(def a :ibig 36N)
(def a :fbig 3.14M)
(def a :str "abc")
(def a :regex #"[0-9]+")
(def a :list '(:a, 1, {:a 2}, [[1, 2], [3, 4]], ("a" "b" :a))) ; difficult
(def a :v[i32][3] [1, 2, 3])
(def a :m[sym,str] {:a "a", :b "b"})
(def a :s[sym] #{:a, :b, :c})
(def a :fn (fn [x :i32] -> :i32 (* x x)))

;; Index slicing
; TODO: nested generics is hard to read
; inspired by numpy
(def vec :v[i32][3, 2] [[1, 2], [3, 4], [5, 6]])
([0] vec)
=> [1, 2] :v[i32][2]
([-1] vec)
=> [5, 6]
([0:-1] vec)
=> [[1, 2], [3, 4]] :v[i32][2, 2]
([0, 1] vec)
=> 2
([:, 1] vec)
=> [2, 4, 6] :v[i32][3]

(def hashmap:m[sym,str] {:a "a", :b "b"})
([:a] hashmap)
=> "a"

;; Funciton
;; Default argument
(defn sample [arg1 :i64 inf,
              arg2 :v[f64] [-inf, 2.0, nan]]
              -> :str
      (format "{0}, {1:?}", arg1, arg2))

(sample 64 [1, 2, 3])
=> "64, [1.0, 2.0, 3.0]:v<f64>[3]"

;; Struct
(struct Enemy [hp :i64 100, attack :f64 200])
;; Can use struct as type
(def slime :Enemy {:attack 2, :hp 20})
([:attack] slime)
=> 2
(print slime.attack) ;; Allow this style

;; 変数は変更可能でいいかなぁ

;; Class
(class Animal
  "Animal Class"
  (defn Animal [hp :i32,
                weight :i32]
                -> :nil
    "constructor of Animal"
    (set! this.hp hp)
    (set! this.weight weight))
  (def hp :i32)
  (def weight :i32 32) ;; can set default value
  (defn walk [dist :i32] -> :str
    (set! this.hp (- this.hp dist)))
    (format "walk {0}km, HP: {1}", dist, this.hp))

(class Dog [Animal]  ; inherit from Animal class
  (defn bow []
    (print "bow!")))

(def dog1 (new Dog [100, 200] {:hp 32, :weight 100})) ;; TODO: Dirty
(dog1.walk 2)
=> "walk 2km, HP: 30"
(dog1.bow)
=> nil
"bow"

;; Macro
(macro if-valid
  "Handle validation more concisely"
  [to-validate validations errors-name & then-else]
  `(let [~errors-name (validate ~to-validate ~validations)]
     (if (empty? ~errors-name)
       ~@then-else)))

;; RegExp
(find #"[0-9]+" "aa123a")
=> "123"

(match #"hello, (.*)" "hello, world")
=> ["hello, world", "world"]

;; chain
(-> )
