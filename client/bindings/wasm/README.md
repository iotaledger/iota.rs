# IOTA Client Library - WebAssembly bindings

WebAssembly (Wasm) bindings for TypeScript/JavaScript to the `iota.rs` client library.

## Which bindings to choose?

The `iota.rs` client library also offers dedicated [Node.js bindings](../nodejs). The differences with this package are outlined below.

|               |   Wasm bindings   |   Node.js bindings    |
|:--------------|:-----------------:|:---------------------:|
| Environment   | Node.js, browsers |        Node.js        |
| Installation  |         -         | Rust, Cargo required* |
| Performance   |        ✔️          |          ✔️✔️           |
| Proof-of-work |  Single-threaded  |    Multi-threaded     |
| MQTT          |         ❌        |          ✔️            |
| Stronghold    |         ❌        |          ✔️            |
| Ledger Nano   |         ❌        |          ✔️            |

*Node.js bindings only need to be compiled during `npm install` if a pre-compiled binary is not available for your platform.

**tl;dr: Use the Node.js bindings if you can. The Wasm bindings are just more portable and support browser environments.** 

## Requirements

- One of the following Node.js versions: '16.x', '18.x';
- `wasm-bindgen` (`cargo install wasm-bindgen-cli`);

## Installation

- Using npm:

```bash
$ npm i @iota/client-wasm
```

- Using yarn:

```bash
$ yarn add @iota/client-wasm
```

## Getting Started

After installing the library, you can create a `Client` instance and interface with it.

### Node.js Usage

```javascript
const { Client } = require('@iota/client-wasm/node');

const client = new Client({
    nodes: ['https://api.testnet.shimmer.network'],
});

client.getInfo().then(console.log).catch(console.error);
```

See the [Node.js examples](../nodejs/examples) for more demonstrations, the only change needed is to import `@iota/client-wasm/node` instead of `@iota/client`.

### Web Setup

Unlike Node.js, a few more steps are required to use this in the browser.

The library loads the compiled Wasm file with an HTTP GET request, so the `iota-client-wasm_bg.wasm` file must be copied to the root of the distribution folder.

A bundler such as [webpack](https://webpack.js.org/) or [rollup](https://rollupjs.org/) is recommended.

#### Rollup

- Install `rollup-plugin-copy`:

```bash
npm install rollup-plugin-copy --save-dev
```

- Add the plugin to your `rollup.config.js`:

```js
// Include the copy plugin.
import copy from 'rollup-plugin-copy'

// ...

// Add the copy plugin to the `plugins` array:
copy({
  targets: [{
    src: 'node_modules/@iota/client-wasm/web/wasm/iota-client-wasm_bg.wasm',
    dest: 'public',
    rename: 'iota-client-wasm_bg.wasm'
  }]
})
```

#### Webpack

- Install `copy-webpack-plugin`:

```bash
npm install copy-webpack-plugin --save-dev
```

- Add the plugin to your `webpack.config.js`:

```js
// Include the copy plugin.
const CopyWebPlugin = require('copy-webpack-plugin');

// ...

experiments: {
    // futureDefaults: true, // includes asyncWebAssembly, topLevelAwait etc.
    asyncWebAssembly: true
}

// Add the copy plugin to the `plugins` array:
plugins: [
    new CopyWebPlugin({
      patterns: [
        {
          from: 'node_modules/@iota/client-wasm/web/wasm/iota-client-wasm_bg.wasm',
          to: 'iota-client-wasm_bg.wasm'
        }
      ]
    }),
    // other plugins...
]
```

### Web Usage

```javascript
import { init, Client } from "@iota/client-wasm/web";

init().then(() => {
  const client = new Client({
    nodes: ['https://api.testnet.shimmer.network'],
  });

  client.getInfo().then(console.log).catch(console.error);
}).catch(console.error);

// Default path to load is "iota-client-wasm_bg.wasm", 
// but you can override it by passing a path explicitly.
//
// init("./static/iota-client-wasm_bg.wasm").then(...)
```
