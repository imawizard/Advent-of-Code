(ns main
  (:require [clojure.string :as strings]))

(let [content
      (-> (slurp "C:\\Users\\Timo\\Desktop\\aoc-day4\\input.txt")
          (clojure.string/split #"\n\n"))]
  (println (strings/join "--------" content)))


(comment
(def a (slurp "C:\\Users\\Timo\\Desktop\\aoc-day4\\input.txt"))

a  
  )
