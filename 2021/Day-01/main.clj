(ns main)

(defn part-a [input]
  (->> input
       (partition 2 1)
       (filter #(apply < %))
       (count)))

(defn part-b [input]
  (->> input
       (partition 3 1)
       (map #(apply + %))
       (part-a)))

(let [measurements
      (->> (slurp "input")
           (clojure.string/split-lines)
           (map #(Integer. %)))]

  (println "a:" (part-a measurements))
  (println "b:" (part-b measurements)))
