;; ===== Literal

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
