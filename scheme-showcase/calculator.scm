(begin
  (display "operand [+, -, *, /, %]:")
  (define op (read-line))

  (display "first number:")
  (define a (read))

  (display "second number:")
  (define b (read))

  (display a " " op " " b " =")

  (if (= op "+") (display (+ a b)) )
  (if (= op "-") (display (- a b)) )
  (if (= op "*") (display (* a b)) )
  (if (= op "/") (display (/ a b)) )
  (if (= op "%") (display (% a b)) )
)