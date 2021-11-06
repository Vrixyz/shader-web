# debug shaders

Toy project to try shaders in native and web

## Tech

- `./assets/` folder is copied into project folder (`crates/web` and `crates/native`)
- hot reloading is present, so you can edit copied assets live. ⚠️ they will be replaced by `cargo make` commands
- web build still needs a browser refresh to update assets. 

## Native

`cd crates/native && cargo make native`

## Web

`cd crates/logic && cargo make web && cd ../web && cargo run --bin web`

