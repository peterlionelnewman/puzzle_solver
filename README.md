# Puzzle Solver, by Pete in Rust~!
####################
This project is about learning Rust / my first Rust project. It solcves the
puzzle in the resources folder of this project:
![the puzzle](https://github.com/peterlionelnewman/puzzle_solver/resources/image_of_the_puzzle.jpg?raw=true)

## STRATEGY:
bogo pick and place a randomly flipped and rotated piece along the left most side or top row:

   1. pick 2 random pieces
   2. rotate the board
   3. rotate the pieces
   4. randomly flip the piece
   5. place both pieces such that they minimizes the perimiter of the inside space
   6. fit the piece
