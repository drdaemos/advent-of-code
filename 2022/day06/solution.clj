(ns solution)

(defn marker? [seq]
  (apply distinct? seq))

(defn start-of [input len]
  (loop [i len
         buf input]
    (if (marker? (take len buf))
      i
      (recur (inc i) (drop 1 buf)))))

(def input-test "mjqjpqmgbljsphdztnvjfqwrcgsmlb")

(defn Main []
  (let
   [input (-> "input.txt" slurp (try (catch Exception e input-test)))]
    (println "Part one:" (start-of input 4)) ;; 1848
    (println "Part two:" (start-of input 14)) ;; 2308
    ))(Main)