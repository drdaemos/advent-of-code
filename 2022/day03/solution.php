<?php

function partOne(array $input): int
{
    $sum = 0;
  
    foreach ($input as $line) {
        $comps = str_split($line, strlen($line) / 2);
        $common = array_unique(array_intersect(str_split($comps[0]), str_split($comps[1])));
        $charCode = ord(reset($common));

        $offset = $charCode >= 97 ? 96 : (65 - 27);
        $priority = $charCode - $offset;
        $sum += $priority;
    }

    return $sum;
}

function partTwo(array $input): int
{
    $sum = 0;
  
    for ($i = 0; $i < count($input); $i += 3) {
        $group = array_slice($input, $i, 3);
        $charGroup = array_map(fn ($item) => str_split($item), $group);
        $common = array_unique(array_intersect(...$charGroup));
        $charCode = ord(reset($common));

        $offset = $charCode >= 97 ? 96 : (65 - 27);
        $priority = $charCode - $offset;
        $sum += $priority;
    }

    return $sum;
}

$input = file_exists(__DIR__ . '/input.txt') 
    ? file_get_contents(__DIR__ . '/input.txt') 
    : <<<TEST
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
TEST;

$lines = explode(PHP_EOL, $input);

echo 'Part one: ' . partOne($lines) . PHP_EOL; // 8401
echo 'Part two: ' . partTwo($lines) . PHP_EOL; // 2641
