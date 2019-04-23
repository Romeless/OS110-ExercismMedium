# OS110-ExercismMedium
Exercism Medium Exercise for OS 110 UNJ

The exercises I have choosen:
  - Clock
  - Pascal's Triangle
  - Queen Attack
  - Robot Simulator
  - Scrabble Score
  
  ---------
  
  ## Robot Simulator.
  
  I will try to explain how I finished one of the exercises:
  
  ### The problem
  ---------
  Write a robot simulator.

  A robot factory's test facility needs a program to verify robot movements.

  The robots have three possible movements:

  - turn right
  - turn left
  - advance

  Robots are placed on a hypothetical infinite grid, facing a particular
  direction (north, east, south, or west) at a set of {x,y} coordinates,
  e.g., {3,8}, with coordinates increasing to the north and east.

  The robot then receives a number of instructions, at which point the
  testing facility verifies the robot's new position, and in which
  direction it is pointing.

  - The letter-string "RAALAL" means:
    - Turn right
    - Advance twice
    - Turn left
    - Advance once
    - Turn left yet again
  - Say a robot starts at {7, 3} facing north. Then running this stream
    of instructions should leave it at {9, 4} facing west.

  --------
  
  ### The solution
  Information we get:
  - A robot has directional parameter.
  - A robot has horizontal and vertical locational parameter.
  - A robot can turn left and turn right, changing the directional parameter.
  - A robot can advance--moving 1 grid, either horizontal or vertical depending on the directional parameter.
  
  The last thing is the instructions, which is just a string of the possible movements. 
  
  Based on the information, let's try creating the struct:
  
    '''rust
    pub struct Robot {
      loc_x: i32,
      loc_y: i32,
      direction: Direction,
    }
    '''
    
  From the struct I created, next we know we need to create the Direction struct:
  
    '''rust
    pub enum Direction {
       North,
       East,
       South,
       West,
     }'''
     
  That concludes the struct creation, time to implement it, since Robot::new() is obvious (just create a Robot given parameter x, y, and direction), let's skip to implement turn left and right:
  
  **The Logic** (Basic Logic, actually) : Say the robot turned left, and say it faced North before turning; it means it is now facing West. On the other hand, a robot facing North and turning right will now face East. Meaning, turning left rotate the robot 90 degree counter clock-wise and turning right rotate the 90 degree clock-wise. To translate it to rust programming language, I choose to do it with **match**, because **if** is too much of a hassle.
  
  **The Code**
     
     '''rust
     pub fn turn_right(mut self) -> Self {
       self.direction = match self.direction {
         Direction::North => Direction::East,
         Direction::East => Direction::South,
         Direction::South => Direction::West,
         Direction::West => Direction::North,
       };
       self
     }

     pub fn turn_left(mut self) -> Self {
       self.direction = match self.direction {
         Direction::North => Direction::West,
         Direction::East => Direction::North,
         Direction::South => Direction::East,
         Direction::West => Direction::South,
       };
       self
     }
     '''
     
 To put it simply, check what direction the robot is facing now, and change it based on that. Next, advance
 
 **The Logic** : First, check on its direction. If the robot is facing North, then we add 1 to its vertical location, opposite for South. If the robot is facing East, then we add 1 to its horizontal location, opposite for West. For this, let's use **match** again.
 
 **The Code** :
 
    '''rust
    pub fn advance(mut self) -> Self {
      self.loc_x += match self.direction {
        Direction::East => 1,
        Direction::West => -1,
        _ => 0,
      };
      self.loc_y += match self.direction {
        Direction::North => 1,
        Direction::South => -1,
        _ => 0,
      };
      self
    }
    '''
