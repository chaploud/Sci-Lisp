; Comment

;; ===== Literal
"abc\n"             ; string (:str)
#"[0-9]+"           ; regular expression (:regex)
false               ; false (:bool)
true                ; true (:bool)
nil                 ; nil (means null/None)
-999                ; int64 (:i64)
-3.14e15            ; float64 (:f64)
nan                 ; Not a Number (:f64)
inf                 ; positive infinity (:f64)
-inf                ; negative infinity (:f64)
-0.0                ; negative zero (:f64)
:symbol             ; symbol (:sym)

;; ===== Collection
; comma is treated as whitespace
'(1, "a", :b)       ; list (can contain any type) (:list)
[1.0, 2.0, 3.0]     ; vector (:v[f64][3])
{:a "a", :b "a"}    ; hashmap (:m[sym,str])
#{:a, :b, :c}       ; hashset (:s[sym])

;; ===== Utility
(type [1, 2, 3])        ; show type
(time (+ 1 2))          ; measure processing time
(print {:a 2, :b 3})    ; print any
(printf "{0:02d}kg" 56) ; print format

;; ===== Variable Binding
(def a 2)               ; variable
(const C [1, 2, 3])     ; constant value (can't assgin after)

(let [a 2]              ; bind variable (lexical scope)
  (set! a 3)            ; assign(destructive)
  (print a))            ; print
; -> nil (return evaluated value of last expression)

;; ===== Function
(defn sum [a :i64,      ; define function
           b :i64]      ; must arguments types
           -> i64       ; must return value typs
  (print a b)
  (+ a b))

(def sum                ; bind function using def
  (fn [a :f64,
       b :f64]          ; lambda/anonymous function
       -> f64
    (+ a b)))

;; ===== macro



; comment
2          ; int
2.0        ; float
"string"   ; string
'(2 3 4)   ; quote (list)

;; ===== Basic
(+ 2 3)  ; => 5
(- 2 3)  ; => -1
(* 2 3)  ; => 6
(/ 2 3.0)  ; => 0.666667
(print "lisp")  ; => "lisp" (display lisp)
(print '(1 2 3))  ; => (1 2 3) (display (1 2 3))
(if (<= 3 2) "true!" "false!") ; => "false!"

;; ===== Define function
(defun fact (n)
  (if (<= n 1)
    1
    (* n (fact (- n 1)))
  ))
(print (fact 8))

;; Builtin Functions

;; Errors
