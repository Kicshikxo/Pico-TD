<p align="center">
  <a href="https://github.com/Kicshikxo/Pico-TD">
    <img src="https://raw.githubusercontent.com/Kicshikxo/Pico-TD/main/build/windows/icon.ico" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">Pico TD</h3>

  <p align="center">
    Tower Defence игра в пиксельной стилистике
  </p>
</p>

### <p align="center">[![Stargazers](https://img.shields.io/github/stars/Kicshikxo/Pico-TD?style=social)](https://google.com) ![License](https://img.shields.io/github/license/Kicshikxo/Pico-TD)</p>

## О проекте

Pico TD - это игра в стиле пиксель-арт в жанре tower defense, в которой вы защищаетесь от наступающей вражеской техники с помощью солдат. Концепция игры вдохновлена серией Bloons TD.

Pico TD отличается тактическим геймплеем, в котором вы расставляете своих солдат так, чтобы максимально повысить их эффективность. Игра включает в себя различные классы солдат, типы врагов и несколько уровней, а также возможность загружать собственные уровни по этому [примеру](https://github.com/Kicshikxo/Pico-TD/blob/main/assets/levels/example.ron).

В игре три класса солдат:

-   Обычный солдат: Сбалансированный юнит с умеренным уроном и быстрым темпом стрельбы.
-   Ракетница: боевая единица с высоким уроном по зоне, медленный темп стрельбы.
-   Снайпер: дальнобойный юнит с высоким уроном и медленным темпом стрельбы.

Враги могут быть наземными, морскими или воздушными целями, включая грузовики, танки, беспилотники, самолеты, лодки и многое другое. Солдаты и враги имеют несколько уровней, что повышает сложность игры по мере её прохождения.

Игра предназначена для игры с помощью мыши, но также поддерживает ввод с сенсорного экрана.

## Инструменты для создания

<p>При создании проекта были использованы следующие инструменты:</p>

-   Rust 1.85.0
-   Bevy 0.15.3

## Установка и запуск

### Установка Rust

Для начала нужно установить [Rust](https://www.rust-lang.org/), если он ещё не установлен. Для этого перейдите по следующей [ссылке](https://www.rust-lang.org/tools/install) и следуйте инструкциям по установке.

### Клонируйте репозиторий

```shell
git clone https://github.com/Kicshikxo/Pico-TD.git
cd Pico-TD
```

### Нативный запуск

Для запуска проекта на нативной платформе:

```shell
cargo run --features bevy/dynamic_linking
```

Для сборки проект в режиме релиза на нативной платформе:

```shell
cargo build --release
```

Чтобы уменьшить размер итогового файла, можно использовать [UPX](https://github.com/upx/upx):

```shell
upx --best --lzma target/release/pico_td
```

### Запуск под WebAssembly (WASM)

Чтобы запустить проект под WASM, вам нужно установить целевую платформу wasm32-unknown-unknown:

```shell
rustup target install wasm32-unknown-unknown
```

Для запуска проекта нужно установить [wasm-server-runner](https://github.com/jakobhellermann/wasm-server-runner):

```shell
cargo install wasm-server-runner
```

```shell
cargo run --target wasm32-unknown-unknown
```

Для сборки проекта в режиме релиза нужно установить [wasm-bindgen-cli](https://github.com/rustwasm/wasm-bindgen):

```shell
cargo install wasm-bindgen-cli
```

```shell
cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --out-name pico_td --out-dir wasm --target web target/wasm32-unknown-unknown/release/pico_td.wasm
```

Для дополнительной оптимизации полученного файла можно использовать [wasm-opt](https://github.com/WebAssembly/binaryen).

```shell
cargo install wasm-opt
```

```shell
wasm-opt -Oz --output wasm/pico_td_bg.wasm wasm/pico_td_bg.wasm
```

## Лицензия

Распространяется по лицензии WTFPL. Смотрите [LICENSE](https://github.com/Kicshikxo/Pico-TD/blob/main/LICENSE.md) для большей информации.
