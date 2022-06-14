(ns main)

(defn part-a [input]
  (reduce
   (fn [[horz depth] [dir x]]
     (case dir
           "forward" [(+ horz x) depth]
           "up"      [horz (- depth x)]
           "down"    [horz (+ depth x)]))
   [0 0] input))

(defn part-b [input]
  (reduce
   (fn [[horz depth aim] [dir x]]
     (case dir
           "forward" [(+ horz x) (+ depth (* aim x)) aim]
           "up"      [horz depth (- aim x)]
           "down"    [horz depth (+ aim x)]))
   [0 0 0] input))

(let [commands
      (->> (slurp "input")
           (clojure.string/split-lines)
           (map #(clojure.string/split % #"\s+"))
           (map (fn [[dir x]] [dir (Integer. x)])))]

  (println "a:"
           (let [[horz depth]
                 (part-a commands)]
             (* horz depth)))
  (println "b:"
           (let [[horz depth]
                 (part-b commands)]
             (* horz depth))))
