# Puzzle Solver, by Pete in Rust~!
####################
This project is about learning Rust / my first Rust project. It solcves the
puzzle in the resources folder of this project:
![the puzzle](https://github.com/peterlionelnewman/puzzle_solver/blob/main/resources/image_of_puzzle.jpg)

## What does it do?
bogo pick and place a randomly flipped and rotated piece along the left most side or top row:

   1. pick 2 random pieces
   2. rotate the board
   3. rotate the pieces
   4. randomly flip the piece
   5. place both pieces such that they minimizes the perimiter of the inside space
   6. fit the pieces
   7. repeat till you run out of pieces
   8. Hope that it solves, if it does, Yay! if not try again.
