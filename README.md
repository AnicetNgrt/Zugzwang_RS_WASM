# Zugzwang Game

*Actix Rust + SSR + React + Wasm*

## What is that?

A web fullstack strategy game project made of:
- `zzg`, a Rust library which aims to implement game logic and AI
- `web/client`, a web frontend for the game
    - Typescript React app bundled with Webpack
    - amazing styles with Tailwindcss V3
    - can be server-side rendered by running `web/client/src/ssr.tsx` and serving the string result
    - the server-side rendered page is then hydrated in the browser by calling `web/client/src/index.tsx`
    - Webpack can also run a no-ssr dev version with fast hot-reload
    - the web app can call `zzg_wasm` a rust wasm library which consists of wasm bindings of `zzg`. It is currently injected in `web/client/src/index.tsx` to avoid using it in SSR because `rust_ssr` does not support WASM
- `web`, an `actix-rs` http server
    - does server-side rendering of `web/client` using `ssr_rs` which internally uses `rusty_V8` a Rust implementation of the V8 Javascript engine
    - serves static assets and bundled wasm from `web/build`
    - aims to be the backend of the game (player accounts, leaderboards, multiplayer...)

## Instructions

- Install `./scripts/install.sh`
- Watch and serve w/o ssr (fast reload) `./scripts/dev_front.sh`
- Watch and serve w/ ssr (slow reload) `./scripts/dev_ssr.sh`
- Run in production `./scripts/run.sh`

## Todo

- [ ] Replace Webpack with Parcel once Tailwind v3 works properly with it again
- [ ] Dig `rust_ssr` and `rusty_V8` to find ways of calling WASM in the server-side rendered page

## Credits

The `web` and `web/client` parts are heavily based on the [Reactix](https://github.com/Valerioageno/reactix) repository by [Valerio Ageno](https://github.com/Valerioageno)