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

## Download (windows only)

https://drive.google.com/file/d/14TijK5Ahylhzvkt0MRjomdC6v6uQC7kN/view?usp=sharing

## build it from source (always fresh)

### Windows

1. install cargo (rust pakage manager) and git (source control)
2. open a terminal in the location you wish to store the source code in
3. type in: git clone https://github.com/MoistPyro/land-calc.git land-calc
4. type in: cd land-calc
4. run "build.bat" as admin

### Linux

1. install cargo and git
2. run: git clone https://github.com/MoistPyro/land-calc.git land-calc
3. run: cd land-calc
4. run: cargo build -r

## get it from crates.io (sometimes outdated)

1. install cargo
2. create a folder for the code to live in
3. in the terminal: 'cargo add land-calc' from inside the folder
4. run build.bat as admin

# errors

if you see '429: too many requests' or something similar in the console while runnung this, stop the program and report the issue to me.
not stopping will get you IP banned from www.scryfall.com.

# acknowledgments:

Frank Karsten, for doing all the math.
https://www.channelfireball.com/article/How-Many-Lands-Do-You-Need-in-Your-Deck-An-Updated-Analysis/cd1c1a24-d439-4a8e-b369-b936edb0b38a/

# CHANGELOG

## v0.1.1 - 22.1.2024

initial release!