 (set-logic LIA)
 (assert (forall ((x Int)) (> x 0)))
 (assert (not (forall ((y Int)) (> y 0))))
 (check-sat)
