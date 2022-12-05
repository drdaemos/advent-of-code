(ns solution
  (:require [clojure.string :as str]))

(defn not-empty-line? [line]
  #_{:clj-kondo/ignore [:unresolved-var]}
  (not (str/blank? (str line))))

(defn split-input-parts [lines]
  (split-with not-empty-line? lines))

(defn transpose [& xs]
  (apply map list xs))

(defn has-letters [string]
  (some #(Character/isLetter %) string))

(defn clean-stack [string]
  (->> string
       (remove #(= \space %))
       drop-last))

(defn parse-stacks [lines]
  (->> lines
       (apply transpose)
       (filter has-letters)
       (mapv clean-stack)))

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
    (println "Part one:" (-> input split-input-parts (solve false))) ;; ZBDRNPMVH
    (println "Part two:" (-> input split-input-parts (solve true)))  ;; WDLPFNNNB
    ))(Main)