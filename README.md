Intel 8080 and space invaders emulator
===========

This is a toy project to learn how to make a simple emulator in rust. The game is playable and the
8080 emulator handles every opcode.

There are two crates :
 - A library for the 8080 processor emulation in [intel-8080-emu](./intel-8080-emu/README.md)
 - The space invaders emulator itself in [space-invaders](./space-invaders/README.md)

There is no requirements for `intel-8080-emu` crate but see the 
[space invaders readme](./space-invaders/README.md) for `space-invaders` itself. In particular, **the
provided ROM is a dummy one** which you should replace by your own.

The repository itself is a cargo workspace so you can run `cargo run -p space-invaders` at the 
root to start the game. 

References 
-------
 - http://www.emulator101.com 
 - http://computerarcheology.com/Arcade/SpaceInvaders
 - The *Intel 8080 Microcomputer Systems User's Manual*
 - *8080 Assembly Langage Programming Manual*
 