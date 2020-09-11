#!/bin/sh

YEAR=$1
DAY=$2

DIR="src/$YEAR/day$DAY"
MAIN_FILE=$DIR/main.rs
echo "Creating new dir"
mkdir -p $DIR

cat >> Cargo.toml <<EOL
[[bin]]
name = "${YEAR}day${DAY}"
path = "$MAIN_FILE"
EOL

sed "s/<YEAR>/$YEAR/g;s/<DAY>/$DAY/g" src/templates/main.rs > $MAIN_FILE

aocdl -day $DAY -year=$YEAR -output "$DIR/input"
