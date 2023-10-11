<?php
error_reporting(E_ALL);
ini_set("display_errors", true);

function part1(int ...$vals)
{
    for ($i = 0; $i < count($vals) - 1; $i++) {
        for ($j = 1; $j < count($vals); $j++) {
            if ($vals[$i] + $vals[$j] == 2020) {
                echo $vals[$i] * $vals[$j], PHP_EOL;
                break 2;
            }
        }
    }
}

function part2(int ...$vals)
{
    for ($i = 0; $i < count($vals) - 2; $i++) {
        for ($j = 1; $j < count($vals) - 1; $j++) {
            for ($k = 2; $k < count($vals); $k++) {
                if ($vals[$i] + $vals[$j] + $vals[$k] == 2020) {
                    echo $vals[$i] * $vals[$j] * $vals[$k], PHP_EOL;
                    break 3;
                }
            }
        }
    }
}

$vals = array_map(function($v) { return intval($v); }, file("../input01.txt"));
part1(...$vals);
part2(...$vals);
