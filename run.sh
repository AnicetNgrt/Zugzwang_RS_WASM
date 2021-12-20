cd zzg_wasm/

wasm-pack build --out-dir ../web/client/node_modules/zzg

cd ../web/client/

npm run build

cd ../..

cargo build --release

cargo run -p web --release