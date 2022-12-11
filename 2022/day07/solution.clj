(ns solution
  (:require [clojure.string :as string]))

#_{:clj-kondo/ignore [:unresolved-var]}
(defn output? [line]
  (not (string/starts-with? line "$ ")))

#_{:clj-kondo/ignore [:inline-def]}
(defn group-entries [input]
  (loop [entries []
         lines input]
    (if (empty? lines)
      entries
      (recur
       (conj entries [(first lines) (take-while output? (rest lines))])
       (drop (inc (count (take-while output? (rest lines)))) lines)))))

#_{:clj-kondo/ignore [:unresolved-var]}
(defn read-input [file default]
  (->> file
       slurp
       (try (catch Exception e default))
       string/split-lines
       group-entries))

(defn parse-cd [command]
  (last (string/split command #"\s")))

(defn change-dir [cwd path]
  (case path
    ".." (pop cwd)
    "/" []
    (conj cwd path)))

(defn to-int [str]
  (some->> str
           (re-find #"[\d.]+")
           Integer/parseInt))

(defn sum-filesizes [ls]
  (->> ls
       (map to-int)
       (remove nil?)
       (reduce +)))

#_{:clj-kondo/ignore [:unresolved-var]}
(defn replay-entry [[cwd stats] [command output]]
  (cond
    (string/starts-with? command "$ cd") [(change-dir cwd (parse-cd command)) stats]
    (string/starts-with? command "$ ls") [cwd (assoc stats (str "/" (string/join "/" cwd)) (sum-filesizes output))]))

(defn dir-sizes [entries]
  (last (reduce
         replay-entry
         [[] {}]
         entries)))

#_{:clj-kondo/ignore [:unresolved-var]}
(defn recur-size [dirs path]
  (->> dirs
       (filter #(string/starts-with? (key %) path))
       vals
       (reduce +)))

(defn recur-dir-sizes [dirs]
  (reduce-kv
   (fn [m k _] (assoc m k (recur-size dirs k)))
   {}
   dirs))

(defn part-one [dirs]
  (->> dirs
       (filter #(<= (val %) 100000))
       vals
       (reduce +)))

(defn part-two [dirs total required]
  (let [to-free (- required (- total (dirs "/")))]
    (->> dirs
         vals
         sort
         (filter #(>= % to-free))
         first)))

(def input-test "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k")

(defn Main []
  (let
   [dirs (recur-dir-sizes (dir-sizes (read-input "input.txt" input-test)))]
    (println "Part one:" (part-one dirs)) ;; 1141028
    (println "Part two:" (part-two dirs 70000000 30000000)) ;; 8278005
    ))(Main)