# USAGE

This **will** try to connect to the internet. spesificly https://api.scryfall.com for validating cards.
the program looks up each card individualy, and each request returns between 8-800 kb of data.
(you have to *try* to get anything more then ~9 tho)

1. paste your list of spells into list.txt (lands will *hopefully* be ignored)
2. start the program.
3. do you have a companion and/or a commander?
4. fill in the three text fields:
- Deck size:    the desired final deck size (99 for EDH, 60 for most other formats)
- ramp + dorks: the total number of *cheap* ramp and acceleration in the list
- cheap draw:   the total number of cantrips and early-game card selection

# Installation

## build it from source

1. install cargo (rust pakage manager)
2. run "build.bat" (windows) or "build.sh" (linux) as admin (windows) or sudo (linux)

## get it from crates.io

1. install cargo
2. in the terminal: cargo install land-calc

# errors

if you see '429: too many requests' or something similar in the console while runnung this, stop the program and report the issue to me.
not stopping will get you IP banned from www.scryfall.com.

# acknowledgments:

Frank Karsten, for doing all the math.
https://www.channelfireball.com/article/How-Many-Lands-Do-You-Need-in-Your-Deck-An-Updated-Analysis/cd1c1a24-d439-4a8e-b369-b936edb0b38a/

# CHANGELOG

## v0.1.1 - 22.1.2024

initial release!