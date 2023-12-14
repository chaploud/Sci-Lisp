; Comment

;; ===== Literal
"abc\n"             ; string
#"[0-9]+"           ; regular expression
false               ; false
true                ; true
nil                 ; nil
-999                ; i64
-3.14e15            ; f64
nan                 ; Not a Number
inf                 ; positive infinity
-inf                ; negative infinity
-0.0                ; negative zero
:keyword            ; keyword symbol
'symbol             ; symbol

;; falsy value is only false and nil
;; "", '(), [], {}, #{}, 0, nan => all truthy

;; ===== Type
; str               ; string
; regex             ; regular expression
; bool              ; boolean
; nil               ; nil
; i64               ; integer 64bit
; f64               ; float 64bit
; key               ; keyword symbol
; list              ; list
; v[T]              ; vector
; m[K,V]            ; hashmap
; s[T]              ; hashset
; fn                ; function
; class             ; class
; struct            ; struct
; enum              ; enum
; macro             ; macro
; special           ; special form

;; ===== Collection
; comma is treated as whitespace
'(1, "a", :b)       ; list (can contain any type)
[1.0, 2.0, 3.0]     ; vector
{:a "a", :b "a"}    ; hashmap
#{:a, :b, :c}       ; hashset

;; ===== Utility
(type [1, 2, 3])             ; show type
(time (+ 1 2))               ; measure processing time
(print {:a 2, :b 3})         ; print any
(printf "{0:02d}kg" 56)      ; print format

;; ===== Variable Binding
(def a :str "abcde")         ; variable
(const C :v[i64] [1, 2, 3])  ; constant value (can't assgin after)

([0:2] a)                    ; slicing => "ab"
([-1] C)                     ; back => 3

(let [a :i64 2]              ; bind variable (lexical scope)
  (set! a 3)                 ; assign(destructive)
  a)                         ; => 3

;; ===== Function
(defn sum [a :i64,           ; define function
           b :i64]           ; must arguments types
           -> i64            ; must return value typs
  (print a b)
  (+ a b))

(def sum :fn                 ; bind function using def
  (fn [a :f64,
       b :f64]               ; lambda/anonymous function
       -> f64
    (return (+ a b))))       ; can use return

;; ===== Control Flow
(if (< 2 3)                  ; if
  "true"                     ; true form
  "false")                   ; false form (must)

(when (< 2 3)                ; when
  (do                        ; true form
    (print "2 < 3")          ; do multiple expressions
    "retval"))

(cond                        ; cond
  (< n 0) "negative"         ; (condition) (expresson)
  (> n 0) "positive"
  :else "default")           ; :else (expression)

(def val :str "hoge")
(switch val                  ; switch
  ["a"]                      ; match "a"
    (print "A")
  ["b", "c"]                 ; match "b" or "c"
    (print "B or C")
  :default                   ; :default (expression)
    (print "DEFAULT"))

(for [i (range 5)]           ; for loop, range
  (print i))

(def a :i64 0)
(while (< a 100)             ; while loop
  (print a)
  (set! a (+ a 1))
  (if (> a 50)
     (break)                 ; break
     (continue)))            ; continue

;; ===== enum
(enum Grade
  [ECONOMY,                  ; => 0
   BUSINESS,                 ; => 1
   FIRST])                   ; => 2

(def your-grade :Grade FIRST)

;; ===== struct
(struct Enemy
  [hp :i64 100,
   attack :f64 200])

(def slime :Enemy {:attack 2, :hp 20})
([:attack] slime)       ; => 2
(print slime.attack)    ; allow this style

;; ===== class
(class Animal
  "Animal Class"        ; docstring

  ;; constructor
  (defn Animal [hp :i32,
                weight :i32]
                -> nil
    "constructor of Animal"
    (set! self.hp hp)
    (set! self.weight weight))

  ;; member
  (def hp :i32)
  (def weight :i32 32)
  (defn walk [dist :i32] -> str
    (set! self.hp (- self.hp dist)))
    (format "walk {0}km, HP: {1}", dist, self.hp))

(class Dog [Animal]     ; inherit from Animal class
  (defn bow []
    (print "bow!")))

(def dog1 :Dog (new Dog [100, 200]))
(dog1.walk 2)           ; => "walk 2km, HP: 30"
(dog1.bow)              ; => nil

;; ===== macro
(macro my-and
  "Evaluates exprs one at time,
   from left to right."           ; docstring
  ([] true)                       ; arguments
  ([x :str] x)                    ; multi arity
  ([x :str & next]                ; variable length argument (& rest)
    `(let [and# :str ~x]          ; quote(`) and unquote(~)
       (if and#
         (my-and ~@next)          ; unquote splicing
         and#))))

(my-and "a" "b" "c")              ; => "c"

;; ===== Built-in Functions

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
(type [1, 2, 3])              ; show type
(time (+ 1 2))                ; measure processing time
(print {:a 2, :b 3})          ; print any
(printf "{0:02d}kg" 56)       ; print format

;; String
(format "{0:03d} kg" 56)      ; format string
(len "abcde")                 ; length of string
(join ["1", "2", "3"] ",")    ; join (=> "1,2,3")
(in? "a" "12aabc32")          ; is string in string?
(upper "abc")                 ; upper-case
(lower "DEF")                 ; lower-case

;; Regular Expression
(find #"[0-9]+" "aa123a")             ; => "123"
(match #"hello, (.*)" "hello, world") ; => ["hello, world", "world"]

;; Vector
(shape [[1, 2], [3, 4], [5, 6]])      ; shape of vector (=> [3, 2])
(len [1, 2, 3])                       ; length of vector
(sum [1, 2, 3])                       ; sum of vector
(mean [1, 2, 3])                      ; mean of vector
(max [1, 2, 3])                       ; max of vector
(min [1, 2, 3])                       ; min of vector
(in? 2 [1, 2, 3])                     ; is element in vector?
(some? [false, true, false])          ; return true if some true
(every? [false, true, false])         ; return true if all true
(sort [3, 1, 2])                      ; sort (non-destructive)
(shuffle [3, 1, 2])                   ; shuffle (non-destructive)
(push [3, 1, 2] 4)                    ; push_back (non-destructive)
(cons [3, 1, 2] 4)                    ; push_front (non-destructive)

(def v [3, 1, 2])
(sort! v)                             ; sort (destructive)
(shuffle! v)                          ; shuffle (destructive)
(push! v 4)                           ; push_back (destructive)
(cons! v 4)                           ; push_front (destructive)
