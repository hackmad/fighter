# Fighter

Based on [JavaScript Fighting Game Tutorial with HTML Canvas](https://www.youtube.com/watch?v=vyqbNFMDRGQ)

Assets:
- [Oak Woods Assets](https://brullov.itch.io/oak-woods)
- [Fighter Asset #1](https://luizmelo.itch.io/martial-hero)
- [Fighter Asset #2](https://luizmelo.itch.io/martial-hero-2)
- [m6x11 a 6px by 11px font by Daniel Linssen](https://managore.itch.io/m6x11)

## Building

This project uses Cargo workspaces and split into 2 library and one binary:

__Notes:__
- A clean build will take a long time due to compilation of the entire
  `bevy` engine.
- The assets folder in the root of the project is symlinked into the
  `native` project.
- The assets folder needs to be copied to `native` and `browser/www` folders.
  It is ignored in `.gitignore` for this reason.

## game 

This is the shared library that houses the core game logic. It will be 
built as part of the other projects which target different runtimes.

## native

This compiles a native binary for the game.

```bash
cargo build -p native
```

To build release version:

```bash
cargo build -p native --release
```

## browser

This is for the WebAssembly game and uses [wasm-pack](https://github.com/rustwasm/wasm-pack).
It can't be build form the workspace root so you have to change to the
project folder first.

__NOTE:__ This relies on `Node 16.13.0 LTS`.

The following will build with `--release` by default:

```bash
cd browser
wasm-pack build
```

The following steps are for the first time setup only, 

Initialize the Node application:

```bash
npm init wasm-app www
```

Install dependencies for Node application:

```bash
cd www
npm install
```

Update `browser/www/package.json`:

```json
{
  // ..
  "scripts": {
    // ...
    "start": "cp -r ../../assets ./ && webpack-dev-server" // Update this line!
    // ...
  },
  // ...
  "devDependencies": {
    "browser": "file:../pkg", // Add this line!
    // ...
  }
  // ...
}
```

Update `browser/www/index.js`:

```javascript
import * as wasm from "browser";
wasm.run()
```

Install dependencies again:

```bash
cd www
npm install
```

## Running

## native

```bash
cp -r assets native/assets
cargo run -p native
```

## browser

```bash
cd browser/www
npm run start
```

If you are using a newer verison of Node, then you might need to add
`NODE_OPTIONS` like this:

```bash
cd browser/www
NODE_OPTIONS=--openssl-legacy-provider npm run start
```

Open the browser to the [http://localhost:8080](http://localhost:8080/)
