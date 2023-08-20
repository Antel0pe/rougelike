# Daily Project Log

dd/mm/yy   

11/08/23    
- Tracking what entities are in a tile     
- Creating components/systems to deal and track damage    
- Letting players attack and kill monsters by running into them    
- When monsters die they are cleared from the screen   
         
12/08/23            
- Monsters hit player :(
- Expanding RunState enum and running systems accordingly
- CHASING DOWN BUGS!!
  - Monsters not chasing player
    - Turns out since I had initially mistakenly added the BlocksTile component to the player, each time the monster tried to pathfind to the player it found that the player's position was blocked. I narrowed down the cause of this bug by printing how many steps away the player was from the monster and always finding that a path couldn't be found. Then I figured that when it was the Monster's RunState, it could never find a path but it could if it was the Player's RunState. This led me to investigate the order the systems were running in and found that if I ran the MapIndexingSystem before the MonsterAISystem, even if it was in the Player's RunState, a path still couldn't be found. This narrowed it down to the MapIndexingSystem and quickly found it was the BlocksTile component that was the issue. 
  - Monsters moving on top of player
    - Fairly easy fix. The MonsterAISystem let the monster move to a position even if it was right next to the player. Changed it to only move when it was more than 1 step away. 


13/08/23
- Resizing map to make space for small user interface
  - While resizing the map, I came across a bug that would generate rooms beyond the borders of the window. When a player went into that room, it would crash the program because they would try to enter a position outside of the window. Had to muck around a bit, figure out why it was being generated the way it was and how to remove the fov so I could test if it was fixed. Simple fix was to reduce the range the room's x, y coords could be generated in.
- Added player health bar to gui
- Added gamelog so any component could add a message to be read by the player
  - Logged when enemies do damage and die

14/08/23
- Added tooltip that displays names of entities when mouse hovers over them
- Refactored + extracted spawning into spawning.rs
- Randomized how many monsters spawn in a room + monster location


15/08/23
- Added health potion item + spawning
- Picking up items
- Listing inventory in GUI

16/08/23
- Drink health potion and restore hp

17/08/23
- Drop items
- Refactor render order of entities

18/08/23
- Refactoring health potion to generalized ProvidesHealing

19/08/23
- Generalized WantsToDrinkPotion into WantsToUseItem
- Added ranged targeting + inflicts damage component
- Display ranged targeting
- Consumables can have multiple charges now

20/08/23
- Added AOE component and spell fireball that ONE SHOTS mobs MUAHAHAHAHA
- Added Confusion component and spell to prevent monsters from attacking for a few turns
- Added dash boots with move speed modifiers