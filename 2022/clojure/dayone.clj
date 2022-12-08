(ns dayone
  (:require [clojure.string :as str]))

(defn count-calories [data]
  (reduce (fn [xs x] 
            (if (nil? x)
              (conj xs 0)
              (update xs (- (count xs) 1) + x)))
          [0] 
          data))

(def input
  (->>
    (slurp "../input/day1.txt")
    (str/split-lines)
    (map #(if (str/blank? %)
            nil
            (Integer/parseInt %)))
    (count-calories)))

(defn part1 [input]
  (apply max input))

(defn part2 [input]
  (reduce + (take 3 (sort > input))))

(println (part1 input))
(println (part2 input))
