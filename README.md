# Conway's Game of Life

## Running the simulator

The only requirement is [Rust](https://www.rust-lang.org/tools/install) to build the simulator and a videocard that supports OpenGL.

To run the simulator run the command
`cargo run`

## Controls

### Keyboard

* **Spacebar**: Pause/resume the simulation
* **1-5**: Set simulation speed
* **R**: Reset the simulation with a random pattern
* **C**: Reset the simulation and clear the board

### Mouse

When the simulation is paused, use the **left mouse** button to activate cells and the **right mouse** button to clear cells.

## TODO

- [x] Add grid
- [x] Spacebar to start (and toggle running)
- [x] R to reset / C to clear
- [x] Center new patterns
- [x] Wrap around board
- [x] Click to add life when paused
- [x] Add active cell counter
- [x] Add speed to window title
- [x] Improve color scheme
- [ ] Add some comments to the code
- [ ] Test on Windows
- [ ] Add screenshot to README
