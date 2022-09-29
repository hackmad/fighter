# Fighting Game

Based on [JavaScript Fighting Game Tutorial with HTML Canvas](https://www.youtube.com/watch?v=vyqbNFMDRGQ)

Assets:
- [Oak Woods Assets](https://brullov.itch.io/oak-woods)
- [Fighter Asset #1](https://luizmelo.itch.io/martial-hero)
- [Fighter Asset #2](https://luizmelo.itch.io/martial-hero-2)

## Building

This project uses Cargo workspaces and split into 2 library and one binary:

__Notes:__
- A clean build will take a long time due to compilation of the entire
  `bevy` engine.
- The assets folder in the root of the project is symlinked into the
  `native` project.
- The assets folder needs to be copied to `browser/www` folder. It is 
  ignored in `.gitignore` for this reason.

## game 

This is the shared library that houses the core game logic. It will be 
built as part of the other projects which target different runtimes.

## native

This compiles a native binary for the game.

```bash
cargo build -p native
```

## browser

This is for the WebAssembly game and uses [wasm-pack](https://github.com/rustwasm/wasm-pack).
It can't be build form the workspace root so you have to change to the
project folder first.

```bash
cd browser
wasm-pack build
```

### First time setup only

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

Update `browser/www/package.json`:

```json
```

Install dependencies again:

```bash
cd www
npm install
```

## Running

## native

```bash
cargo run -p native
```

## browser

Run the server (`NODE_OPTIONS` is necessary due to an issue with Node
18.8.0 and OpenSSL).

```bash
cd browser/www
NODE_OPTIONS=--openssl-legacy-provider npm run start
```

Open the browser to the [http://localhost:8080](http://localhost:8080/)

