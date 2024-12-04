#!/bin/sh

# Part 1
grep -Eo 'mul\([0-9]{1,3},[0-9]{1,3}\)' ../input/day3.txt | tr '\n' '+' | sed -E 's/mul\(([0-9]{1,3}),([0-9]{1,3})\)/(\1\*\2)/g' | sed 's/+$/\n/g' | bc

# Part 2
grep -Eo "mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)" ../input/day3.txt | tr '\n' '+' | sed -E -e "s/do\(\)/S/g" -e "s/don't\(\)/E/g" -e "s/E[^S]*S//g" -e "s/[SE]//g" -e "s/\+\+/\+/g" -e 's/mul\(([0-9]{1,3}),([0-9]{1,3})\)/(\1\*\2)/g' | sed 's/+$/\n/g' | bc
