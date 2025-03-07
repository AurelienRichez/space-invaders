#!/usr/bin/env bash 

rm -rf ./build
wasm-pack build --target web --out-dir build/pkg
cp web/* build