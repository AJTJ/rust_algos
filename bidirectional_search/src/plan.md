

- we have 2d grid and start coords and end coords
- performing a BFS from start point and end point to achieve an efficient computational time
- keep track of the start parents and end parents to retrace the path
- Do we need a separate hashmap to keep track of cells visited?
  - No, we can check the parents list
- we need to perform a simultaneous step for both the start and end BFS'

- And after each step, we check if the current cell we are on is currently being investigated by the other BFS or it if it exists in the other parents list
- from there, we gather the coordinates in order
  - This might mean reversing one of the lists of parents
- print out a list of coordinates

NOTES
- I forgot to check the places I've visited already in each BFS