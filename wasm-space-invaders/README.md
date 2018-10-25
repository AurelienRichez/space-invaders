This program emulates a space invader arcade game from 1978 with web asssembly.

The code is packaged with [wasm-pack](https://rustwasm.github.io/wasm-pack/), and installed in a 
small web project as a dependency in `www` folder. You also need npm.

```
# in this folder 
wasm-pack build
cd www
npm install
npm run start
```

the the space invader game is served on localhost:8080. 

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