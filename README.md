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

# errors

if you see '429: too many requests' or something similar in the console while runnung this, stop the program and report the issue to me.
not stopping will get you IP banned from www.scryfall.com.

# stuff this will eventualy do

1. read a mtg deck list
2. search scryfall to find mana value and type of card
3. calculate average mana value
4. use calculation to figure out required number of lands
5. make use of bulk data files to cut out / lessen need for internet (might not be worth)

# acknowledgments:

Frank Karsten, for doing all the math.
https://www.channelfireball.com/article/How-Many-Lands-Do-You-Need-in-Your-Deck-An-Updated-Analysis/cd1c1a24-d439-4a8e-b369-b936edb0b38a/

# TODO

- use bulk data to not require internet?
- github