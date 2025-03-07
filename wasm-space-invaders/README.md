This program emulates a space invader arcade game from 1978 with web asssembly.

The code is packaged with [wasm-pack](https://rustwasm.github.io/wasm-pack/). You need to have it installed
The project can be built via `./build.sh`. This will create the webapp in the `build` folder.

You can then serve the build folder for instance 

```
cd build 
python3 -m http.server # serve it on http
```


the space invader game is served on localhost:8080. 

Or you can also try it directly on [github pages](https://aurelienrichez.github.io/space-invaders/)
Commands
-------
 
 | key         | function                  |
 |-------------|---------------------------|
 | enter       | insert coin               |
 | ctrl left   | start game with 1 player  |
 | ctrl right  | start game with 2 player  |
 | space       | fire                      |
 | arrow right | go right                  |
 | arrow left  | go left                   |



 TODO
 ----
  - Add sound