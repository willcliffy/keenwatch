# Keenwatch

## Art style

8x8 style

## Game theory

- 2v2 moba
- Goal is to go down the opponent’s ladder.
  - Flags open up parts of the map and allow you to progress?
    - alternately, players can open parts of enemy base/maze but it takes time
  - best of three with different maps?
    - Outcome of round 1 determines starting positions of round 2?

- Async turn based movement + combat
  - count down timer on everyone’s screen.
  - Everyone has 3 seconds to move/attack, but within that “frame” everyone takes their turn asynchronously.

- Powerups can increase your ability to move/deal damage for the rest of that round.
  - Powerups can be permanent/round/temporary.
  - Can you keep powerups in an inventory and use them? Or maybe only some? They should probably be discarded at the end of the round.
  - Types
    - Speed buff - move extra tiles each turn
    - Damage buff - attacks deal more damage
    - Strength buff - more attacks/abilities per turn

- Types of characters
  - MELEE - damage and tenacity. Weak to RANGE, strong against TANK
  - RANGE - speed and range. Weak to TANK, strong against RANGE
  - TANK  - structures and crowd control. Weak to MELEE, strong against RANGE

## Order of operations

### v0.0.0

~~Map~~, camera, movement
MELEE character implementated
Win conditions implemented

### v0.0.1

1st powerup
RANGE character implemented

### v0.0.2

2nd powerup
TANK character implemented (basic)

### v0.0.3

3rd powerup
powerup tuning

### v0.0.4

TANK structure abilities implemented

### v0.0.5

Map revisit - 2nd and 3rd round maps different
