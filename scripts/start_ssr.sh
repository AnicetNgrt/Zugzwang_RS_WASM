#!/bin/bash

fuser -k 8080/tcp

cd zzg_wasm
wasm-pack build --out-dir ../web/client/src/zzg
cd ../web/
rm -rf dist
cd client
npm run build
cd ../..
cargo run -p web --release &
