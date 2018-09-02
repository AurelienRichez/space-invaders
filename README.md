Intel 8080 and space invaders emulator
===========

This is a toy project to learn how to make a simple emulator in rust. The game is playable and the
8080 emulator handles every opcode. I first used gtk to run the graphic interface and then tried
piston, so there are currently 2 implementations of the game.

There are 4 crates :
 - [intel-8080-emu](./intel-8080-emu) : a library for the 8080 processor emulation
 - [space-invaders-core](./space-invaders-core) : common code for space invaders specific emulation
 and asset embedding.
 - [gtk-space-invaders](./gtk-space-invaders) : a space invaders implementation using 
 [gtk-rs](https://gtk-rs.org/)
 - [piston-space-invaders](./piston-space-invaders) : a space invaders implementation using 
 [piston libraries](https://www.piston.rs/)


Requirements
------------

There is no special requirements for `intel-8080-emu` crate (except for the 
[rust toolchain](https://www.rust-lang.org) of course).

**⚠⚠⚠ The real space invaders rom is not included in this repository ⚠⚠⚠** The provided rom 
`space-invaders-core/resources/dummy.rom` runs fine and is useful to test that everything compiles, 
but it just draws random garbage on the screen. 
**You have to add the file `space-invaders-core/resources/invaders.rom`** which can be 
easily found on the internet. The compiler automatically embed `invaders.rom` if it exists.

`gtk-space-invaders` needs the [gtk-rs requirements](http://gtk-rs.org/docs-src/requirements.html) 
(`sudo apt install libgtk-3-dev` on a debian base distribution, `brew install gtk+3` on osx)

The repository itself is a cargo workspace so you can run `cargo run -p space-invaders` at the 
root to start the game. 

Troubleshooting
---------------
> I added `space-invaders-core/resources/invaders.rom` but the emulator keep showing the random 
> stuff.

The compiler automatically embed `invaders.rom` if it exists but you might need to run `cargo clean`
to force a recompilation if you compiled first with `dummy.rom`.


References 
-------
 - http://www.emulator101.com 
 - http://computerarcheology.com/Arcade/SpaceInvaders
 - The *Intel 8080 Microcomputer Systems User's Manual*
 - *8080 Assembly Langage Programming Manual*
 