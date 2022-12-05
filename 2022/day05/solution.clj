(ns solution
  (:require [clojure.string :as str]))

(defn split-input-parts [lines]
   #_{:clj-kondo/ignore [:unresolved-var]}
  (split-with #(not (str/blank? (str %))) lines))

(defn transpose [& xs]
  (apply map list xs))

(defn parse-stacks [lines]
  (mapv
   (fn [stack]
     (drop-last
      (remove #(= \space %) stack)))
   (filter
    (fn [stack] (some #(Character/isLetter %) stack))
    (apply transpose lines))))

(defn get-top-crates [stack n bulk]
  (if bulk
    (take n stack)
    (reverse (take n stack))))

(defn move-crates [stacks n from to bulk]
  (let
   [added (update stacks (- to 1) #(concat %2 %1) (get-top-crates (stacks (- from 1)) n bulk))]
    (update added (- from 1) #(drop n %))))

(defn parse-inst [inst]
  (map
   #(Integer/parseInt %)
   (re-seq #"\d+" inst)))

(defn apply-instructions [stacks instructions bulk]
  (reduce
   (fn [acc inst]
     (apply move-crates (concat [acc] (parse-inst inst) [bulk])))
   stacks
   instructions))

(defn part-one [parts]
  (apply str (map
              first
              (apply-instructions
               (parse-stacks (first parts))
               (rest (last parts))
               false))))

(defn part-two [parts]
  (apply str (map
              first
              (apply-instructions
               (parse-stacks (first parts))
               (rest (last parts))
               true))))

(defn Main [] 
  #_{:clj-kondo/ignore [:unresolved-var]}
  (let
   [input (str/split-lines (slurp "input.txt"))]
    (println "Part one:" (part-one (split-input-parts input))) ;; 
    (println "Part two:" (part-two (split-input-parts input))) ;; 
    ))(Main)