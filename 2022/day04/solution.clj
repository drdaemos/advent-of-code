(ns solution
  (:require [clojure.string :as str]))

(defn to-int-range [str]
  (map #(Integer/parseInt %) (str/split str #"-")))

(defn in-range? [num range]
  (<= (first range) num (last range)))

(defn subset? [rangeA rangeB]
  (and (in-range? (first rangeA) rangeB) (in-range? (last rangeA) rangeB)))

(defn overlaps? [rangeA rangeB]
  (or (in-range? (first rangeA) rangeB) (in-range? (last rangeA) rangeB)))

(defn subset-pair? [rangeA rangeB]
  (or (subset? rangeA rangeB) (subset? rangeB rangeA)))

(defn overlap-pair? [rangeA rangeB]
  (or (overlaps? rangeA rangeB) (overlaps? rangeB rangeA)))

(defn part-one [input]
  (count
   (filter true?
           (map #(apply subset-pair? (map to-int-range (str/split % #","))) input))))

(defn part-two [input]
  (count
   (filter true?
           (map #(apply overlap-pair? (map to-int-range (str/split % #","))) input))))

(defn Main []
  (let [input (str/split-lines (slurp "input.txt"))]
    (println "Part one:" (part-one input)) ;; 582
    (println "Part two:" (part-two input)) ;; 893
    ))(Main)
