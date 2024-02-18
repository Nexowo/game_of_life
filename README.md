# Game of Life
-----------
My personal implementation of <a href = https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life>Conway's Game</a> of life in Rust.
This small project was made to improve my comprehention of the Rust language and arrays/vectors.

You can change parameters:
* `size` to change the size of the board
* `nb_epochs` to change the number of loops that will be executed
* You can add `board[x][y] = 1` where `x` and `y` are values in between `0` (included) and `size` (excluded) where you want `1` to be and suppress those preexisting to do your own initial configuration

To run :
* Make sure that Rust is well installed on your computer (<a href = https://www.rust-lang.org/tools/install>Installation link</a>)
* Make sure that cargo's location is added to your PATH variable
* Open a terminal in the project and type `cargo run`
