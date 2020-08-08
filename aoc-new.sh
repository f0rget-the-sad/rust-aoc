#!/bin/sh

YEAR=$1
DAY=$2

DIR="src/$YEAR/day$DAY"
echo "Creating new dir"
mkdir -p $DIR

#TODO: cp template

aocdl -day $DAY -year=$YEAR -output "$DIR/input"
