# iota.rs-wasm

This is the **alpha** version of the official WASM binding to [IOTA's Rust API](https://github.com/iotaledger/iota.rs).

## Web Setup

Install the library:
```bash
$ npm install @iota/iota-rs-wasm
// or using yarn
$ yarn add @iota/iota-rs-wasm
```

The library loads the WASM file with an HTTP GET request, so the .wasm file must be copied to the root of the dist folder.

### Rollup
- Install `rollup-plugin-copy`:
```bash
$ npm install rollup-plugin-copy
// or using yarn
$ yarn add rollup-plugin-copy
```

- Add the copy plugin usage to the `plugins` array under `rollup.config.js`:
```js
copy({
	targets: [{
		src: 'node_modules/@iota/iota-rs-wasm/wasm-web/iota_wasm_bg.wasm',
		dest: 'public',
		rename: 'iota_client.wasm'
	}]
})
```

### Webpack
- Install `copy-webpack-plugin`:
```bash
$ npm install copy-webpack-plugin --save-dev
// or using yarn
$ yarn add copy-webpack-plugin --dev
```

- Add the copy plugin to the `plugins` array of your webpack config:
```js
new CopyWebpackPlugin([
  {
    from: 'node_modules/@iota/iota-rs-wasm/wasm-web/iota_wasm_bg.wasm',
    to: 'iota_client.wasm'
  }
])
```

## Usage

### Web
```js
import * as client from '@iota/iota-rs-wasm/web'

const uri = 'https://nodes.comnet.thetangle.org'
await client.addNode(uri)
client.getNodeInfo().then(nodeInfo => { ... })
```

### Node.js
```js
const client = require('@iota/iota-rs-wasm/node')

const uri = 'https://nodes.comnet.thetangle.org'
await client.addNode(uri)
client.getNodeInfo().then(nodeInfo => { ... })
```
