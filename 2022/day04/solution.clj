(ns day04
  (:require [clojure.spec.alpha :as s]))

(defn toIntRange [str]
    (map #(Integer/parseInt %) (clojure.string/split str #"-"))
)

(defn inRange [num range]
    (and (>= num (first range)) (<= num (last range)))
)

(defn isSubset [rangeA rangeB]
    (and (inRange (first rangeA) rangeB) (inRange (last rangeA) rangeB))
)

(defn overlap [rangeA rangeB]
    (or (inRange (first rangeA) rangeB) (inRange (last rangeA) rangeB))
)

(defn isSubsetPair [a b]
    (def rangeA (toIntRange a))
    (def rangeB (toIntRange b))
    (or (isSubset rangeA rangeB) (isSubset rangeB rangeA))
)

(defn isOverlapPair [a b]
    (def rangeA (toIntRange a))
    (def rangeB (toIntRange b))
    (or (overlap rangeA rangeB) (overlap rangeB rangeA))
)

(defn partOne [input]
    (count 
        (filter true? 
            (map #(apply isSubsetPair (clojure.string/split % #",")) input)))
)

(defn partTwo [input]
    (count 
        (filter true? 
            (map #(apply isOverlapPair (clojure.string/split % #",")) input)))
)

(defn Main []
    (def input (clojure.string/split-lines (slurp "input.txt")))
    (println "Part one:" (partOne input)) ;; 582
    (println "Part two:" (partTwo input)) ;; 582
)(Main)
