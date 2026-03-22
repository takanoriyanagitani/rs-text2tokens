#!/bin/sh

WASM="./rs-text2tokens.wasm"

input(){
    echo 'The quick brown Foxes were jumping!'
    echo 'Hello, world!'
}

input |
    wasmtime run "${WASM}"
