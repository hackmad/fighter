# Fighter

A 2-player fighting game written in Rust with the Bevy game engine.
Based on [JavaScript Fighting Game Tutorial with HTML Canvas](https://www.youtube.com/watch?v=vyqbNFMDRGQ)

Assets:
- [Oak Woods Assets](https://brullov.itch.io/oak-woods)
- [Fighter Asset #1](https://luizmelo.itch.io/martial-hero)
- [Fighter Asset #2](https://luizmelo.itch.io/martial-hero-2)
- [m6x11 a 6px by 11px font by Daniel Linssen](https://managore.itch.io/m6x11)
- [Melee sounds](https://opengameart.org/content/3-melee-sounds)
- [Boss Battle #6 Metal](https://opengameart.org/content/boss-battle-6-metal)
- [Adventure Intro Title](https://opengameart.org/content/adventure-intro-title-cinematic-epic)

## Building

This project uses Cargo workspaces and split into 2 library and one binary:

__Notes:__
- A clean build will take a long time due to compilation of the entire `bevy` engine.
- The assets folder in the root of the project is symlinked into the `desktop` project.
- The assets folder needs to be copied to `destop` and `browser/www` folders. It is ignored in `.gitignore` for this reason.

## game 

This is the shared library that houses the core game logic. It will be built as part of the other projects which target
different platforms such as desktop and WebAssembly.

## desktop

This compiles a desktop binary for the game.

```bash
cargo build -p desktop
```

To build release version:

```bash
cargo build -p desktop --release
```

## browser

This is for the WebAssembly game and uses [wasm-pack](https://github.com/rustwasm/wasm-pack). It can't be build form the
workspace root so you have to change to the project folder first.

__NOTE:__ This relies on `Node 16.13.0 LTS`.

The following will build with `--release` by default:

```bash
cd browser
wasm-pack build --out-dir pkg
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

## desktop

```bash
cp -r assets desktop/assets
cargo run -p desktop
```

## browser

```bash
cd browser/www
npm run start
```

If you are using a newer verison of Node, then you might need to add `NODE_OPTIONS` like this:

```bash
cd browser/www
NODE_OPTIONS=--openssl-legacy-provider npm run start
```

Open the browser to the [http://localhost:8080](http://localhost:8080/).

__NOTES:__

By default browsers block autoplay and so there might be no sound. 

Firefox: 
- Under `Tools` menu click `Page Info`.
- Click `Permissions` tab.
- Under `Autoplay` uncheck `Use Default`
- Check `Allow Audio and Video`.

Safari:
- Go to `Preferences`.
- Click `Websites` tab.
- On the left side click `Auto-Play`
- Enable `Allow All Auto-Play` for `localhost`.

Chrome:
- Go to `Settings`.
- On the left select `Privacy and security`.
- Scroll down to `Additional content settings`.
- Click `Sounds`
- Under `Allowed to play sound` add `localhost`.
