## Yew&Bevy&tTailwind

A yew project importing a bevy wasm file.

## Setup

Install wasm32 rust target:

```
rustup target add wasm32-unknown-unknown
```

Install [wasm-pack](https://github.com/rustwasm/wasm-pack):

```
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Install the [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) cli:

```
cargo install wasm-bindgen-cli
```

In case off _ssl_ error type the following in your terminal or use node 16.

```
export NODE_OPTIONS=--openssl-legacy-provider
```

## Demo

https://sunny-cobbler-b61d05.netlify.app/
