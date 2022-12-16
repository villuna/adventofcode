(ns dayfifteen
  (:require [clojure.string :as str]))

(defn manhattan 
  "Returns the manhattan distance between two points"
  [[x1 y1] [x2 y2]]
  (+ (abs (- x2 x1)) (abs (- y2 y1))))

(defn diamond 
  "The \"diamond\" of size r around a tile s is every tile that is
  a manhattan distance of <= r away from s."
  [beacon sensor]
  {:centre sensor :size (manhattan sensor beacon)})

(defn parse-line [line]
  (let [groups (rest (re-matches #"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)" line))
        coords (vec (map #(Integer/parseInt %1) groups))]
    {:sensor (subvec coords 0 2) :beacon (subvec coords 2 4)}))

(defn get-interval 
  "Finds the interval containing all the tiles in the given row that
  are in the given diamond, or nil if it is empty."
  [diamond row]
  (let [[xs ys] (:centre diamond)
        size (:size diamond)
        d (- size (abs (- ys row)))
        [x y] [(- xs d) (+ xs d)]]
    (if (>= y x)
      [x y]
      nil)))

(def input 
  (->>
    (slurp "../input/day15.txt")
    (str/split-lines)
    (map parse-line)))

(defn intervals-at-row 
  "Looks through the positions of sensors and beacons and returns which
  tiles in row `row` are within the \"diamond\" around each sensor.
  Returns a vector of disjoint intervals [a, b] in lexicographic order."
  [input row]
  (->>
    input
    (map #(diamond (:beacon %) (:sensor %)))
    (map #(get-interval % row))
    (filter some?)
    (sort)
    ; This next function turns the vector of overlapping intervals into
    ; disjoint ones (that cover the same area). It looks a bit arcane.
    ; Trust me, it works.
    (reduce (fn [is [a2 b2]]
              (if (empty? is)
                (conj is [a2 b2])
                (let [[a1 b1] (last is)]
                  (if (<= a2 (+ 1 b1))
                    (assoc is (- (count is) 1) [a1 (max b1 b2)])
                    (conj is [a2 b2])))))
            [])))

(defn shrink-intervals 
  "Takes in a (sorted, disjoint) vector of intervals and restricts them
  all to be within the range [low, high]."
  [[low high] intervals]
  (if (empty? intervals) 
    []
    (let [filtered (filter (fn [[a b]] (and (>= b low) (<= a high))) intervals)
          [a1 _] (first filtered)
          [_ b2] (last filtered)
          new-low (if (< a1 low) low a1)
          new-high (if (> b2 high) high b2)]
      (->> (vec filtered)
           (#(assoc % 0 (assoc (first %) 0 new-low)))
           (#(assoc % (- (count %) 1) (assoc (last %) 1 new-high)))))))


(defn part-1 [input row]
  (- (apply + (map (fn [[a b]] (+ 1 (- b a))) (intervals-at-row input row)))
     (count (distinct (filter #(= row (nth % 1)) (map :beacon input))))))

(println (part-1 input 2000000))

(defn part-2 [input limit frequency-multiplier]
  (->> (map 
         (fn [row xrange] {:y row :x xrange})
         (range)
         (->>
           (range (+ 1 limit))
           (map #(shrink-intervals [0 limit] (intervals-at-row input %)))))
       (filter #(not= [[0 limit]] (:x %)))
       (first)
       (#(assoc % :x (+ 1 (last (first (:x %))))))
       (#(+ (:y %) (* frequency-multiplier (:x %))))))

(println (part-2 input 4000000 4000000))
