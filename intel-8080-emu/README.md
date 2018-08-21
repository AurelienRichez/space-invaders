Intel 8080 emu
--------

An intel 8080 emulator library (another one). Initially implemeted to build a space invaders 
emulator (another another one).

This library provides provides utilities to parse 8080 binary and simulate an [8080 microprocessor](https://en.wikipedia.org/wiki/Intel_8080). All op codes are implemented and there is no external 
dependencies.

TODO 
----
 - API doc
 - Maybe better memory handling. It is currently a simple `Box<[u8]>` and does not distinguish 
 between ROM and RAM.