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
'symbol             ; quoted symbol

;; falsy value is only 'false' and 'nil'
;; "", '(), [], {}, #{}, 0, nan => all truthy

;; ===== Collection
; comma is treated as whitespace
'(1, "a", :b)                ; list
[1.0, 2.0, 3.0]              ; vector
{:a "a", :b "b", :c "c"}     ; map (holds the insersion order)
#{:a, :b, :c}                ; set (holds the insersion order)

;; ===== Function Call
(type [1, 2, 3])             ; show type
(time (+ 1 2))               ; measure processing time
(print [1, 2] "abc\n" 123)   ; print any

;; ===== Variable Binding
;; NOTE: Different from Clojure
(def a "abcde")              ; variable (mutable, 'local' scope)
(const C [1, 2, 3])          ; constant value (immutable)

([0|2] a)                    ; slice => "ab"
(-1 C)                       ; back => 3

(let [a 2]                   ; bind variable (local scope)
  (set! a 3)                 ; assign(destructive)
  a)                         ; => 3

;; ===== Function
(defn my-sum [a b]           ; define function
 "sum two value"             ; docstring
  (print a b)
  (+ a b))

(def my-sum                  ; bind function using def
  (fn [a b]                  ; anonymous/lambda function
    (return (+ a b))         ; can use early return
    (- a b)))

;; ===== Control Flow
(if (< 2 3)                  ; if
  true                       ; true form
  false)                     ; false form (must)

(when (< 2 3)                ; when
  (do                        ; true form
    (print "2 < 3")          ; do multiple expressions
    "retval"))

(cond                        ; cond
  (< n 0) "negative"         ; (condition) (expresson)
  (> n 0) "positive"
  :else "default")           ; :else (expression)

(def val "hoge")
(switch val                  ; switch
  ["a"]                      ; match "a"
    (print "A")
  ["b", "c"]                 ; match "b" or "c"
    (print "B or C")
  :default                   ; :default (expression)
    (print "DEFAULT"))

(for [i (range 5)]           ; for loop, range
  (print i))

(def a 0)
(while (< a 10)              ; while loop
  (print a)
  (set! a (+ a 1))
  (if (> a 5)
    (break (+ a 9994))       ; break with return value
    (continue))              ; continue
  (print "never print"))

;; ******************* WIP **********************
;; ===== enum
(enum Grade                       ; define enum
  "Grade Enum"                    ; docstring
  [ECONOMY,
   BUSINESS,
   FIRST])

(def your-grade Grade.FIRST)      ; allow this style

;; ===== struct
(struct Enemy                     ; define struct
  "Enemy Struct"                  ; docstring
  [hp,
   attack])

(def slime
  (Enemy {:attack 2, :hp 20}))    ; using struct
([:attack] slime)                 ; access member => 2
(print slime.attack)              ; allow this style

;; ===== class
(class Animal                     ; define class
  "Animal Class"                  ; docstring

  ;; constructor
  (defn Animal [hp, weight]
    "constructor of Animal"
    (set! self.hp hp)
    (set! self.weight weight))

  ;; member
  (def hp)
  (def weight)
  (defn walk [dist]
    (set! self.hp (- self.hp dist)))
    (format "walk {0}km, HP: {1}",
      dist, self.hp))

(class Dog [Animal]               ; inherit from Animal class
  (defn bow []
    (print "bow!")))

(def dog1 (Dog [100, 200]))       ; instanciate class
(dog1.walk 2)                     ; => "walk 2km, HP: 98"
(dog1.bow)                        ; => nil

;; ===== macro
(macro my-and                     ; define macro
  "Evaluates exprs one at time,
   from left to right."           ; docstring
  ([] true)                       ; multi arity
  ([x] x)
  ([x & next]                ; variable length argument (& rest)
    `(let [and# ~x]          ; quote(`) and unquote(~)
       (if and#                   ; auto-gensym(xxx#)
         (my-and ~@next)          ; unquote splicing(~@)
         and#))))

(my-and "a" "b" "c")              ; => "c"
;; ******************************************

;; ===== Built-in Functions/Macros

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
(= 2 2 2)            ; equal
(!= 2 3)             ; not equal
(< 2 3)              ; less
(<= 2 3)             ; less equal
(> 2 3)              ; greater
(>= 2 3)             ; greater equal

;; Logical
(and true false)     ; and
(or true false)      ; or
(xor true true)      ; xor
(not false)          ; not

;; Predicates
(nil? nil)
(true? true)
(false? false)
(number? 1)
(i64? 1)
(f64? 1.0)
(zero? 0)
(even? 2)
(odd? 3)
(empty? [])
(string? "abc")
(keyword? :abc)
(symbol? 'abc)
(list? '(1, 2, 3))
(vector? [1, 2, 3])
(map? {:a 1, :b 2})
(set? #{1, 2, 3})

;; Cast
(str 3.14)           ; to string
(str 'abc)
(str :abc)
(i64 "2")              ; to i64
(f64 "2.0")            ; to f64
(list #{1, 2, 3})      ; to list
(vector '(1, 2, 3))    ; to vector
(hmap [:a 1, :b 2])    ; to map
(hset [1, 2, 2])       ; to set

;; Math
(sqrt 2)
(abs -2)             ; absolute value
(cos (* 2.0 *pi*))   ; cosine
(sin (/ *pi* 2))     ; sine
(tan 2.0)            ; tangent
(acos 0.5)           ; arccosine
(asin 0.5)           ; arcsine
(atan 0.5)           ; arctangent
(log 2 10)           ; log_2(10)
(ln *e*)             ; log_e(e), natural logarithm
(log10 2.0)          ; log_10(2.0)
(rand)               ; rondom value 0.0 to 1.0
(randint 0 30)       ; random integer 0 to n

;; Utility
(type [1, 2, 3])              ; show type
(time (+ 1 2))                ; measure processing time
(print {:a 2, :b 3})          ; print any
(printf "{0:03}kg" 56)        ; print format (WIP)

;; String
(len "abcde")                 ; length of string
(join [1, 2, 3] ",")          ; join (=> "1,2,3")
(split "1,2,3" "," i64)       ; split (=> [1, 2, 3])
(replace "abc" "a" "x")       ; replace (=> "xbc")
(concat "abc" "def")          ; concat (=> "abcdef")
(trim " abc ")                ; trim (=> "abc")
(in? "a" "12aabc32")          ; is string in string?
(index "abc" "12aabc32")      ; string index in string (=> 3)
(count "abc" "12aabc32")      ; count string in string (=> 1)
(upper-case "abc")            ; upper-case
(lower-case "DEF")            ; lower-case
(lower-camel "abc_def")       ; lowerCamelCase (=> "abcDef")
(upper-camel "abc_def")       ; UpperCamelCase (=> "AbcDef")
(snake-case "abcDef")         ; snake-case (=> "abc_def")
(kebab-case "abcDef")         ; cebab-case (=> "abc-def")
(title-case "abcDef")         ; Title Case (=> "Abc Def")
(train-case "abcDef")         ; Train-Case (=> "Abc-Def")
(shouty-snake "abcDef")       ; SHOUTY_SNAKE_CASE (=> "ABC_DEF")
(shouty-kebab "abcDef")       ; SHOUTY-KEBAB-CASE (=> "ABC-DEF")
(repeat "abc" 2)              ; repeat string (=> "abcabcabc")
(reverse "abc")               ; reverse (=> "cba")
(format "π: {:.2}" 3.1415)    ; format string (WIP)

;; Regular Expression
(find #"[0-9]+" "aa123a")                ; => "123"
(find-all #"No\.(\d+)" "No.1 No.2 No.3") ; => ["hello, world", "world"]
(replace "aa123a" #"[0-9]{2}" "x${1}y")  ; => "aax12y3a"

;; At
(-1 [1, 2, 3])                        ; back => 3

;; Key Access
(:a {:a 1, :b 2, :c 3})               ; get value by key (keyword)
(0 {0 "a", 1 "b", 2 "c"})             ; get value by key (i64)
("a" {"a" 1, "b" 2, "c" 3})           ; get value by key (string)

;; Slice
([0|2] [1, 2, 3])                     ; slice => [1, 2]
([0|-1|2] "abcdefg")                  ; slice with step => "ace"
([|, 1] [[1, 2], [3, 4], [5, 6]])     ; slice => [2, 4, 6]
([|, 1|2] [[1, 2], [3, 4], [5, 6]])   ; slice (like numpy) => [[2], [4], [6]]

;; Vector
(first [1, 2, 3])                     ; first
(last [1, 2, 3])                      ; last
(rest [1, 2, 3])                      ; rest
(len [1, 2, 3])                       ; length of vector
(sum [1, 2, 3])                       ; sum of vector
(mean [1, 2, 3])                      ; mean of vector
(max [1, 2, 3])                       ; max of vector
(min [1, 2, 3])                       ; min of vector
(in? 2 [1, 2, 3])                     ; is element in vector?
(index 2 [1, 2, 3])                   ; index of element
(index-all 2 [1, 2, 3, 2])            ; all index of element
(some? [false, true, false])          ; return true if some truthy
(every? [false, true, false])         ; return true if all truthy
(sort [3, 1, 2] :asc)                 ; sort
(reverse [3, 1, 2])                   ; reverse
(push [3, 1, 2] 4)                    ; push_back
(cons [3, 1, 2] 4)                    ; push_front
(concat [1, 2, 3] [4, 5, 6])          ; concat
(shuffle [3, 1, 2])                   ; shuffle

(def v [1, 2, 3])
(get v 1)                             ; get value by index
(insert! v 1 999)                     ; insert
(remove! v 0)                         ; remove

;; Map
(keys {:a 1, :b 2, :c 3})             ; keys
(vals {:a 1, :b 2, :c 3})             ; values
(items {:a 1, :b 2, :c 3})            ; key-value pairs

(def m {:b 2, :c 3})
(get m :b)                            ; get value by key
(insert! m :a 1)                      ; insert/replace
(remove! m :a)                        ; remove

;; Set
(def s1 #{2 3})
(get s1 2)                            ; get key
(insert! s 1)                         ; insert/replace
(remove! s 1)                         ; remove
(def s2 #{1 2})
(union s1 s2)                         ; union
(intersect s1 s2)                     ; intersect
(difference s1 s2)                    ; difference

;; Functional Programming
; partial
; map
; filter
; reduce
; repeat
; chunk
; ->
; ->>

;; === WIP
;; try-catch-finally
;; MultiArity(Autodoc)
;; Polars binding
;; shape
;; Destructuring
;; Parallel
;; SIMD
;; JIT
