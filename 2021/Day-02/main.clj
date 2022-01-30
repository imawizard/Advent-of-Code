(ns main)

(defn part-a [input]
  (reduce
   (fn [[pos depth] [dir units]]
     (case dir
           "forward" [(+ pos units) depth]
           "up"      [pos (- depth units)]
           "down"    [pos (+ depth units)]))
   [0 0] input))

(defn part-b [input]
  (reduce
   (fn [[pos depth aim] [dir units]]
     (case dir
           "forward" [(+ pos units) (+ depth (* aim units)) aim]
           "up"      [pos depth (- aim units)]
           "down"    [pos depth (+ aim units)]))
   [0 0 0] input))

(let [commands
      (->> (slurp "input")
           (clojure.string/split-lines)
           (map #(clojure.string/split % #"\s+"))
           (map (fn [[dir units]] [dir (Integer. units)])))]

  (println "a:"
           (let [[pos depth]
                 (part-a commands)]
             (* pos depth)))
  (println "b:"
           (let [[pos depth]
                 (part-b commands)]
             (* pos depth))))
