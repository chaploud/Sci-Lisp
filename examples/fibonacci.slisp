(defn fibonacci [n]
  "Compute for the nth fibonacci number."
  (if (or (zero? n) (= n 1))
    1
    (let [f1 (fibonacci (- n 1))
          f2 (fibonacci (- n 2))]
      (+ f1 f2))))

(print (fibonacci 10))
