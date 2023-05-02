# Crosswords Rainbow Solver

This is a solver for "Crosswords Rainbow", the 7th task of the 1st episode of the [enigame - enimal kingdom](https://enigame.de) puzzle hunt from 2023-04-28:

## The Task
![Crossword Puzzle](https://enigame.de/public/file/event/45/episode/136/episodetask/5896/crosswords.png)

 **across**
 ```
 3. rabbit
10. ladybug
11. batoid
12. hamster
13. chicken
14. skunk
16. duck
17. alligator
18. platypus
19. peacock
20. alpaca
```

**down**
```
1. armadillo
2. cow
4. snake
5. zebra
6. dolphin
7. raccoon
8. hedgehog
9. bat
12. penguin
15. dromedary
```

**words**
```
beetle
bird
bread
chicken
danger
dice
disco
duck
fart
flap
flap
floof
flurry
formal
giraffe
horse
hot
land
leather
log
milk
moose
mouse
murder
ocean
panda
pigeon
piglet
pony
potato
puppy
radar
rat
rope
sea
sheep
spiky
squirrel
tank
thief
tiger
trash
turkey
```

## How to Solve It
The crossword puzzle needs to be filled with meme names of animals, based on the puzzle hints (e.g. seagull = beach chicken).
The colored squares gives the final solution (in rainbow order).

## Implemented Features
- brute force search via backtracking
- order crosswords gaps based on BFS on the crossword "graph" to ensure that every gap except the first has at least one letter already filled in (reduces branching factor)
- use bitvectors to maintain used words
- queries of the form "all possible word combinations of length $n$ that have letter $x$ at position $i$" are precomputed and stored in a three dimensional lookup table (indexed by $n$, $i$, $x$).


## Possible Futures Features
- print final solution word for each possible crossword solution
- further algorithmic optimizations (e.g. excluding already used words could speed things up significantly for deep recursion layers).
- there might be a way to improve caching behavior
- constraints: try to solve puzzle with some known words
- Multithreading
- remove the deliberate leaking of memory in the beginning of the algorithm (this way the borrow checker doesn't get in the way since all possible words have a `'static` lifetime).