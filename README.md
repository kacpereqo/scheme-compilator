# Scheme compilator
 a turing complete scheme compilator written entirely in rust, the best programing language of all time

---
## Example
Fibonacci sequence
```scheme
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
```

----
## Syntax
what's done and what will be done in future (never)
\
&nbsp;

### Logical operators
- ☑️ == 
- ☑️ !=
- ☑️ <
- ☑️ <=
- ☑️ >
- ☑️ >=
- 🟦 and
- 🟦 or
- 🟦 not
-------------------


### Mathetmatical operators
- ☑️ + 
- ☑️ -
- ☑️ *
- ☑️ /
- ☑️ %
- 🟦 ^
-------------------

### Keywords 
- ☑️ begin 
- ☑️ define
- ☑️ if
- ☑️ while
- 🟦 break
- ☑️ for 
- ☑️ read
- ☑️ read-line
- ☑️ #t
- ☑️ #f
- 🟦 import
- 🟦 functions
-------------------

### Supported types
- ☑️ String 
- ☑️ Int 64
- ☑️ Float 64 
- ☑️ Bool
- 🟦 Array
-------------------
