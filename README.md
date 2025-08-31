<p align="center">
  <a href="https://github.com/Kicshikxo/Pico-TD">
    <img src="https://raw.githubusercontent.com/Kicshikxo/Pico-TD/main/build/windows/icon.ico" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">Pico TD</h3>

  <p align="center">
    Pixel-art Tower Defense Game
  </p>

  <p align="center">
    [ English | <a href="https://github.com/Kicshikxo/Pico-TD/blob/main/README.ru.md">Русский</a> ]
  </p>
</p>

### <p align="center">[![Stargazers](https://img.shields.io/github/stars/Kicshikxo/Pico-TD?style=social)](https://google.com) ![License](https://img.shields.io/github/license/Kicshikxo/Pico-TD)</p>

## About the Project

Pico TD is a pixel-art tower defense game where you defend against waves of enemy vehicles using soldiers. The concept is inspired by the Bloons TD series.

Pico TD features tactical gameplay where you place your soldiers to maximize their efficiency. The game includes different soldier classes, enemy types, multiple levels, and the ability to load custom levels following this [example](https://github.com/Kicshikxo/Pico-TD/blob/main/assets/levels/example.ron).

There are three soldier classes:

-   **Regular Soldier:** A balanced unit with moderate damage and a fast rate of fire.
-   **Rocket Launcher:** High area damage, but a slow rate of fire.
-   **Sniper:** Long-range, high-damage unit with a slow rate of fire.

Enemies can be ground, naval, or air units, including trucks, tanks, drones, planes, boats, and more. Both soldiers and enemies have multiple levels, increasing the difficulty as the game progresses.

The game is designed to be played with a mouse but also supports touch input.

## Play Online

You can play the game directly in your browser:

-   [kicshikxo.itch.io/pico-td](https://kicshikxo.itch.io/pico-td)
-   [pico-td.kicshikxo.ru](https://pico-td.kicshikxo.ru)

## Built With

The following tools were used to build this project:

-   Rust 1.88.0
-   Bevy 0.16.1

## Installation and Run

### Install Rust

First, install [Rust](https://www.rust-lang.org/) if you haven't already. Follow the installation instructions here: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

### Clone the Repository

```shell
git clone https://github.com/Kicshikxo/Pico-TD.git
cd Pico-TD
```

### Native Run

Run the project on your native platform:

```shell
cargo run --features bevy/dynamic_linking
```

Build the project in release mode:

```shell
cargo build --release
```

To reduce the final binary size, you can use [UPX](https://github.com/upx/upx):

```shell
upx --best --lzma target/release/pico_td
```

### Run with WebAssembly (WASM)

To run the project with WASM, install the `wasm32-unknown-unknown` target:

```shell
rustup target install wasm32-unknown-unknown
```

Install [wasm-server-runner](https://github.com/jakobhellermann/wasm-server-runner) to run the project

```shell
cargo install wasm-server-runner
```

```shell
cargo run --target wasm32-unknown-unknown
```

For a release build, install [wasm-bindgen-cli](https://github.com/rustwasm/wasm-bindgen):

```shell
cargo install wasm-bindgen-cli
```

```shell
cargo build --release --target wasm32-unknown-unknown
```

```shell
wasm-bindgen --out-name pico_td --out-dir wasm --target web --no-typescript target/wasm32-unknown-unknown/release/pico_td.wasm
```

For additional optimization, use [wasm-opt](https://github.com/WebAssembly/binaryen).

```shell
cargo install wasm-opt
```

```shell
wasm-opt -Oz --all-features --output wasm/pico_td_bg.wasm wasm/pico_td_bg.wasm
```

## Run on Android

To build for Android, install the required targets:

```shell
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
```

And install [cargo-ndk](https://github.com/bbqsrc/cargo-ndk):

```shell
cargo install cargo-ndk
```

Then build the game for all platforms:

```shell
cargo ndk -t armeabi-v7a -t arm64-v8a -t x86 -t x86_64 -o android/app/src/main/jniLibs/ build --release --link-libcxx-shared
```

Finally, open the `android` folder in Android Studio and build the project.

## External Assets

-   [Kenney Game Assets](https://kenney.nl/assets/)

## License

Distributed under the WTFPL License. See [LICENSE](https://github.com/Kicshikxo/Pico-TD/blob/main/LICENSE.md) for more information.
