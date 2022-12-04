(ns solution
  (:require [clojure.string :as str]))

(defn to-int-range [str]
  (map #(Integer/parseInt %) (str/split str #"-")))

(defn in-range [num range]
  (<= (first range) num (last range)))

(defn is-subset [rangeA rangeB]
  (and (in-range (first rangeA) rangeB) (in-range (last rangeA) rangeB)))

(defn overlap [rangeA rangeB]
  (or (in-range (first rangeA) rangeB) (in-range (last rangeA) rangeB)))

(defn is-subset-pair [rangeA rangeB]
  (or (is-subset rangeA rangeB) (is-subset rangeB rangeA)))

(defn is-overlap-pair [rangeA rangeB]
  (or (overlap rangeA rangeB) (overlap rangeB rangeA)))

(defn part-one [input]
  (count
   (filter true?
           (map #(apply is-subset-pair (map to-int-range (str/split % #","))) input))))

(defn part-two [input]
  (count
   (filter true?
           (map #(apply is-overlap-pair (map to-int-range (str/split % #","))) input))))

(defn Main []
  (let [input (str/split-lines (slurp "input.txt"))]
    (println "Part one:" (part-one input)) ;; 582
    (println "Part two:" (part-two input)) ;; 893
    ))(Main)
