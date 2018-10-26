Space invaders emulator
===========

This is a toy project to learn how to make a simple emulator in rust. I first used gtk to run the graphic interface, then tried piston, and finally took a shot at webassembly. So there are currently
3 implementations of the game (2 native and one in the browser).
Every implementation is based on [intel-8080-emu](https://github.com/AurelienRichez/intel-8080-emu)
which is a library I extracted from this project.

There are 4 crates :
 - [space-invaders-core](./space-invaders-core) : common code for space invaders specific emulation
 and asset embedding.
 - [gtk-space-invaders](./gtk-space-invaders) : a space invaders implementation using 
 [gtk-rs](https://gtk-rs.org/)
 - [piston-space-invaders](./piston-space-invaders) : a space invaders implementation using 
 [piston libraries](https://www.piston.rs/)
 - [wasm-space-invaders](./wasm-space-invaders) : a space invaders implementaion using the 
 webassembly target of rust and a simple canvas in js.


Requirements
------------

**⚠⚠⚠ The real space invaders rom is not included in this repository ⚠⚠⚠** The provided rom 
`space-invaders-core/resources/dummy.rom` runs fine and is useful to test that everything compiles, 
but it just draws random garbage on the screen. 
**You have to add the file `space-invaders-core/resources/invaders.rom`** which can be 
easily found on the internet. The compiler automatically embed `invaders.rom` if it exists.

`gtk-space-invaders` needs the [gtk-rs requirements](http://gtk-rs.org/docs-src/requirements.html) 
(`sudo apt install libgtk-3-dev` on a debian base distribution, `brew install gtk+3` on osx)

`wasm-space-invaders` needs [wasm-pack](https://rustwasm.github.io/wasm-pack/) and [npm](https://www.npmjs.com/).

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
 