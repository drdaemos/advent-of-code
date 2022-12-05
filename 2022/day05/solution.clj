(ns solution
  (:require [clojure.string :as str]))

(defn split-input-parts [lines]
  #_{:clj-kondo/ignore [:unresolved-var]}
  (->> lines
       (split-with
        #(-> % str str/blank? not))))

(defn transpose [& xs]
  (apply map list xs))

(defn parse-stacks [lines]
  (mapv
   (fn [stack]
     (->> stack
          (remove #(= \space %))
          drop-last))
   (->> lines
        (apply transpose)
        (filter (fn [line] (some #(Character/isLetter %) line))))))

(defn get-top-crates [stack n bulk]
  (if bulk
    (take n stack)
    (->> stack (take n) reverse)))

(defn move-crates [input bulk [n from to]]
  (as-> input stacks
    (update stacks (- to 1) #(concat %2 %1) (get-top-crates (stacks (- from 1)) n bulk))
    (update stacks (- from 1) #(drop n %))))

(defn parse-inst [inst]
  (->> inst
       (re-seq #"\d+")
       (map #(Integer/parseInt %))))

(defn apply-instructions [stacks instructions bulk]
  (reduce
   (fn [acc inst]
     (->> inst
          parse-inst
          (move-crates acc bulk)))
   stacks
   instructions))

(defn solve [parts bulk]
  (as-> "" tops
    (apply-instructions (parse-stacks (first parts)) (rest (last parts)) bulk)
    (map first tops)
    (apply str tops)))

(defn Main []
  #_{:clj-kondo/ignore [:unresolved-var]}
  (let
   [input (-> "input.txt" slurp str/split-lines)]
    (println "Part one:" (solve (split-input-parts input) false)) ;; ZBDRNPMVH
    (println "Part two:" (solve (split-input-parts input) true))  ;; WDLPFNNNB
    ))(Main)