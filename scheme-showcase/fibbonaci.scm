(begin  
  (display "Enter a number: ")
  (define n (read))
  (define i 1)

  (define a 1)
  (define b 1)

  (while (!= n 0) (begin
    (display "fib(" i ") = " a "\n")
    (define next (+ a b))
    (define a b)
    (define b next)
    (define n (- n 1))
    (define i (+ i 1))
    )
  )
)