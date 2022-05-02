# Talent Path Generator
> for World of Warcraft: Dragonflight

This code is based on the similar python implementation from [simc_support](https://github.com/Bloodmallet/simc_support/blob/feature/10-0-experiments/simc_support/game_data/new_talent.py).

This implementation managed to be slower than the python implementation. I'm looking for help to figure out why and to fix this.

## Basic flow
1. load talents from json into a vector
2. initialize talents (figure out the indexes of their children and parents in the vector)
3. plant a tree (struct, that contains the talents and a paths vector of a vector holding as many booleans as talents exist, each bool represents if the talent at the same index is selected, at the start contains only one completely unselected, read false, path)
4. grow the tree until x points were spent
    1. collect talent indexes that don't require parents as entry points
    2. create a vector of tuples (path, entry points)
    3. until `points` are spent
        - create new paths and their available entry points
        - prevent duplicates
    4. return the final paths vec
