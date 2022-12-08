(ns solution)

(require '[clojure.string :refer [split-lines]])

(defn to-int [str]
  (Integer/parseInt str))

(defn read-grid [file]
  (->> file
       slurp
       split-lines
       (mapv #(clojure.string/split % #""))
       (mapv #(mapv to-int %))))

(defn at [grid [x y]]
  ((grid y) x))

(defn has? [grid [x y]]
  (and (contains? grid y) (contains? (grid y) x)))

(defn at-border? [grid [x y]]
  (or
   (== x 0)
   (== y 0)
   (== x (dec (count (grid 0))))
   (== y (dec (count grid)))))

(defn up [[x y]]
  [x (dec y)])

(defn right [[x y]]
  [(inc x) y])

(defn down [[x y]]
  [x (inc y)])

(defn left [[x y]]
  [(dec x) y])

(defn next-tree [grid [x y]]
  (if (= (inc x) (count (grid y)))
    [0 (inc y)]
    [(inc x) y]))

(defn cast-visible? [grid f pos]
  (let [height (at grid pos)]
    (if (at-border? grid pos)
      true
      (loop [pos pos]
        (if (has? grid (f pos))
          (if (<= height (at grid (f pos)))
            false
            (recur (f pos)))
          true)))))

(defn cast-scenic [grid f pos]
  (let [height (at grid pos)]
    (if (at-border? grid pos)
      0
      (loop [pos pos
             score 0]
        (if (has? grid (f pos))
          (if (<= height (at grid (f pos)))
            (inc score)
            (recur (f pos) (inc score)))
          score)))))

(defn raycast [grid compfn rayfn pos]
  (compfn
   (rayfn grid up pos)
   (rayfn grid right pos)
   (rayfn grid down pos)
   (rayfn grid left pos)))

(defn count-visible [grid]
  (loop [pos [0 0]
         count 0]
    (if (has? grid pos)
      (recur (next-tree grid pos) (if (raycast grid (fn [& x] (some true? x)) cast-visible? pos) (inc count) count))
      count)))

(defn count-scenic [grid]
  (loop [pos [0 0]
         highscore 0]
    (if (has? grid pos)
      (recur (next-tree grid pos) (max (raycast grid * cast-scenic pos) highscore))
      highscore)))

(defn Main []
  (let
   [grid (read-grid "input.txt")]
    (println "Part one:" (count-visible grid)) ;; 1560
    (println "Part two:" (count-scenic grid)) ;; 252000
    ))(Main)