use macro_lisp::lisp;

lisp!(defun factorial ((n i32)) i32
    (if (<= n 1)
        1
        (* n (factorial (- n 1)))));

lisp!(defun fibonacci ((n i32)) i32
    (if (== n 1)
        0
        (if (== n 2)
            1
            (+ (fibonacci (- n 1)) (fibonacci (- n 2))))));

lisp!(defun main () ()
    (defconstant num (factorial 10))
    (defconstant num2 (fibonacci 6))
    (println "10! = {}" num)
    (println "fib(6) is {}" num2));
