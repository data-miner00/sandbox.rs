;;; Sum of a list
(defun sum-of-list (lst)
  (reduce #'+ lst))

;;; Usage
(let ((my-list '(1 2 3 4 5)))
  (format t "Sum of the list ~a is ~a" my-list (sum-of-list my-list)))

;;; Ackermann's formula
(defun ackermann (m n)
  (cond
    ((= m 0) (+ n 1))
    ((= n 0) (ackermann (- m 1) 1))
    (t (ackermann (- m 1) (ackermann m (- n 1))))))

;;; Usage
(format t "Ackermann(3, 3) = ~a~%" (ackermann 3 3))