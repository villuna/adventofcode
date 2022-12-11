(ns dayten
  (:require [clojure.string :as str]))

(def width 40)
(def height 6)

(defn parse-op [line]
  (let [groups (re-find (re-matcher #"addx (-?\d+)" line))]
    (if groups
      (Integer/parseInt (nth groups 1))
      nil)))

(defn apply-op [history op]
  (let [signals (:signals history) current (:current history)]
    (if (nil? op)
      ; Noop
      (assoc history :signals (conj signals current))
      ; Addx {op}
      (assoc history
             :current (+ current op)
             :signals (into [] (concat signals [current current]))))))

(def ops (->>
  (slurp "../input/day10.txt")
  (str/split-lines)
  (map parse-op)))

(def history (:signals (reduce apply-op {:signals [] :current 1} ops)))

(def part-1
  (apply + (map #(* % (nth history (- % 1))) [20 60 100 140 180 220])))

(println part-1)

(def part-2
  (->>
    (map (fn [pixel signal]
           (if (and 
                 (<= (mod pixel width) (+ signal 1))
                 (>= (mod pixel width) (- signal 1)))
             "#"
             ".")) 
         (range)
         history)
    (partition width)
    (map #(apply str %))
    (str/join "\n")))

(println part-2)
