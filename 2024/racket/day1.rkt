#lang racket

(define (read-input day)
  (string-trim
    (port->string 
      (open-input-file (format "../input/day~a.txt" day)))))

(define (parse-pair line) 
  (map string->number (string-split line)))

(define pairs (map parse-pair (string-split (read-input 1) "\n")))

(define l1 (sort (map first pairs) <))
(define l2 (sort (map (lambda (x) (first (rest x))) pairs) <))

(define part-1 (apply + (map (lambda (a b) (abs (- a b))) l1 l2)))
(define part-2 (apply + (map (lambda (x) (* x (count (lambda (y) (= x y)) l2))) l1)))

(printf "Part 1: ~a\n" part-1)
(printf "Part 2: ~a\n" part-2)
