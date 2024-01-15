(begin  
  (define i 10)
  (while (!= i 0) (begin
    (display "i^2 = " (* i i) "\n")
    (define i (- i 1))
    )
  )
)