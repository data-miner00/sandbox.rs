;;; Arbitrary list
(defvar code '(print (+ 1 (* 1 2))))
(message "%s = %s"
    code
    (eval code))

(defun myfunc ()
    (message "Hello World"))

(defun my-propaganda ()
    (interactive)
    (insert "enter anything into buffer"))

(defun greet (firstname lastname)
    (format "Hello %s %s" firstname lastname))

(setq fullname (concat "Hi" "world"))

(defun log-diary ()
    (setq filename (concat "~/website/content/diary" (format-time-string "%Y-%m-%d-%H-%M") ".md"))
    (find_file filename)
    (insert (concat "+++\ntitle = \"" (format-time-string "%Y.%m.%d %A %P %H:%M") "\"\n+++\n\n")))
