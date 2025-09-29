# Project

Chip8 implemenation in Rust

## Details

I decided to take on this project as a learning experience both in Rust and Low-level emulation.

## Usage

```shell
cargo run --release /path/to/rom
```

You can download any Chip8-compatible rom and run it using a command above.

I highly recommend to pay a visit (and drop a star) to this repo full of roms:
![click](https://github.com/loktar00/chip8/tree/master/roms)

## TBD

- [ ] Write better unit tests
  - [x] Test Instructions fully
  - [ ] Test CPU
  - [ ] Test Memory
  - [ ] Test Display
  - [ ] Test Keypad
- [x] Implement Display in code
- [ ] Implement Display visually
- [ ] Implement Keypad
- [ ] Document and check for typos
- [ ] Run Space Invaders/Pong
