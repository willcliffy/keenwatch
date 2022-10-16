# hm

Win condition:

- all players on a team have reached the opponent's goal and either destroyed the goal or secured the goal (TBD)
- opponent gate is unlocked and opened

## "Entities"

- Player(s)
- Goal
- Gate
  - Lock
- Creeps? (TBD)

### Player user cases

- Move
- Attack (Combat system TBD)
- Special ability
- Death and respawn

### Goal user cases

- Reached by player
  - When a player enters the goal, `y` occurs
  - When all players on a team enter the goal, `x` occurs
- When the goal is secured (TBD), players may go down the ladder to the next level

### Gate user cases

- There is one game on either side of the map
- Each team needs to open the opponents gate in order to secure the base
- The gate is opened by unlocking both of the gate locks

### Gate lock user cases

- When a player stays in collision with the lock for a certain amount of time, the lock is unlocked
- If both players are on a lock at the same time, both locks are unlocked immediately

## Game Mechanic Events

- Player enters/leaves lock
- Player enters/leaves goal
- Player enters/leaves gate
- Player enters ladder
- Lock is unlocked
- Gate is opened
- Goal is secured

## Combat Mechanic Events

Pre-tick:

- Player registers attack
- Player registers special ability

Post-tick:

- Player receives damage
- Player receives special ability damage
- Player dies
