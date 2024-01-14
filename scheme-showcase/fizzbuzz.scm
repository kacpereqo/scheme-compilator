(begin
  (display "enter number: ")
  (define n (read))

  (if (= (% n 15) 0 ) (display "fizzbuzz") 
    (if (= (% n 3) 0) (display "fizz")
    (if (= (% n 5) 0) (display "buzz")
      (display n)
      )
    )
  )
)