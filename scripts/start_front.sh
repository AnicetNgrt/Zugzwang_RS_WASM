#!/bin/bash

fuser -k 3000/tcp

cd zzg_wasm
wasm-pack build --out-dir ../web/client/src/zzg
cd ../web/client
npm run start &