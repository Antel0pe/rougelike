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