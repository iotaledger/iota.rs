---
description: Getting started with the official IOTA Client Library Wasm binding.
image: /img/logo/libraries.png
keywords:
- Rust
- install
- npm
- system environment variables
---
# Getting Started with Wasm

## Install the library

### NPM

```bash
npm install @iota/client-wasm
```

### Yarn

```bash
yarn add @iota/client-wasm
```

## Build

Alternatively, you can build the bindings if you have Rust installed. If not, refer to [rustup.rs](https://rustup.rs) for installation instructions. Then install the necessary dependencies using:

```npm install```

and then build the bindings for `node.js` with

```npm run build:nodejs```

or for the `web` with

```npm run build:web```

## NodeJS Setup

```js
const iota = require('@iota/client-wasm/node')

async function main() {
  // Get the nodeinfo
  let iota_client = await iota.Client.withNode("https://api.lb-0.h.chrysalis-devnet.iota.cafe/");
  console.log("Nodeinfo: ", await iota_client.getInfo())
}
main()
```

## Web Setup

The library loads the WASM file with an HTTP GET request, so the .wasm file must be copied to the root of the dist folder.

### Rollup

- Install `rollup-plugin-copy`:

```bash
npm install rollup-plugin-copy --save-dev
// or using yarn
yarn add rollup-plugin-copy --dev
```

- Add the copy plugin usage to the `plugins` array under `rollup.config.js`:

```js
// Include the copy plugin
import copy from 'rollup-plugin-copy'

// Add the copy plugin to the `plugins` array of your rollup config:
copy({
  targets: [{
    src: 'node_modules/@iota/client-wasm/web/client_wasm_bg.wasm',
    dest: 'public',
    rename: 'client_wasm_bg.wasm'
  }]
})
```

### Webpack

- Install `copy-webpack-plugin`:

```bash
npm install copy-webpack-plugin --save-dev
// or using yarn
yarn add copy-webpack-plugin --dev
```

```js
// Include the copy plugin
const CopyWebPlugin= require('copy-webpack-plugin');

// Add the copy plugin to the `plugins` array of your webpack config:

new CopyWebPlugin({
  patterns: [
    {
      from: 'node_modules/@iota/client-wasm/web/client_wasm_bg.wasm',
      to: 'client_wasm_bg.wasm'
    }
  ]
}),
```

### Usage

```js
import * as iota from "@iota/client-wasm/web";

iota.init().then(() => {
async function main() {
  // Get the nodeinfo
  let iota_client = await iota.Client.withNode("https://api.lb-0.h.chrysalis-devnet.iota.cafe/");
  console.log("Nodeinfo: ", await iota_client.getInfo())
}
main()
});

// or

(async () => {
  await iota.init()
  // Get the nodeinfo
  let iota_client = await iota.Client.withNode("https://api.lb-0.h.chrysalis-devnet.iota.cafe/");
  console.log("Nodeinfo: ", await iota_client.getInfo())
})()

// Default path is "client_wasm_bg.wasm", but you can override it like this
await iota.init("./static/client_wasm_bg.wasm");
```

`iota.init().then(<callback>)` or `await iota.init()` is required to load the wasm file (from the server if not available, because of that it will only be slow for the first time)
