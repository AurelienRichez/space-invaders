Space invader emulator
-----------------------

This program emulates a space invader arcade game from 1978 with gtk-rs.

Building
--------
I assume the [rust toolchain](https://www.rust-lang.org) is installed.

 - Make sure that you have [gtk-rs requirements](http://gtk-rs.org/docs-src/requirements.html) 
(`sudo apt install libgtk-3-dev` on a debian base distribution, `brew install gtk+3` on osx). 
 - **⚠⚠⚠ The real space invaders rom is not included in this repository ⚠⚠⚠** The provided rom runs
  fine, but just draws random garbage on the screen. **You have to change the file 
  `resources/invaders.rom` for the real rom file.** 
 - Other than that, it is classic `cargo build`, `cargo run` etc.

Known issues
-------
 - For some unknown reason the rendering did not work on osx the last time I tried but I did not 
 investigate further yet (an `out of memory error` occurs in gtk)

Commands
-------
 
 | key         | function    |
 |-------------|-------------|
 | enter       | insert coin |
 | s           | start game  |
 | space       | fire        |
 | arrow right | go right    |
 | arrow left  | go left     |



 TODO
 ----
  - Add player 2 commands
  - Add sound