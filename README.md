# Project

Chip8 implemenation in Rust

## Badge

![badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/schukark/f285310eb2aa23028f1ffa5b6a740d14/raw/doc-coverage.json)

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

- [x] Write better unit tests
  - [x] Test Instructions fully
  - [x] Test CPU
  - [x] Test Memory
  - [x] Test Display
  - [x] Test Keypad
- [x] Implement Display in code
- [x] Implement Keypad
- [x] Test the machine as a whole
- [x] Add logging
- [x] Implement Emulator window
- [ ] Document and check for typos
- [x] Run Space Invaders/Pong
- [x] Add sound
- [x] Fix render lagging
