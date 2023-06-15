<?php
error_reporting(E_ALL);
ini_set("display_errors", true);

function part_a(int ...$vals)
{
    for ($i = 0; $i < count($vals) - 1; $i++) {
        for ($j = 1; $j < count($vals); $j++) {
            if ($vals[$i] + $vals[$j] == 2020) {
                return $vals[$i] * $vals[$j];
            }
        }
    }
    return 0;
}

function part_b(int ...$vals)
{
    for ($i = 0; $i < count($vals) - 2; $i++) {
        for ($j = 1; $j < count($vals) - 1; $j++) {
            for ($k = 2; $k < count($vals); $k++) {
                if ($vals[$i] + $vals[$j] + $vals[$k] == 2020) {
                    return $vals[$i] * $vals[$j] * $vals[$k];
                }
            }
        }
    }
    return 0;
}

$vals = array_map(function($v) { return intval($v); }, file("input"));
echo "a: ", part_a(...$vals), PHP_EOL;
echo "b: ", part_b(...$vals), PHP_EOL;
