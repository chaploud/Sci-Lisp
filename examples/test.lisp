; Comment

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
symbol              ; symbol

; (1, "a", :b)        ; list (can contain any type)
; [1.0, 2.0, 3.0]     ; vector
; {:a "a", :b "a"}    ; hashmap
; #{:a, :b, :c}       ; hashset

; (type [1, 2, 3])             ; show type
; (time (+ 1 2))               ; measure processing time
; (print {:a 2, :b 3})         ; print any
; (printf "{0:02d}kg" 56)      ; print format
