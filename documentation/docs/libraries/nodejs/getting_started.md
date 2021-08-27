# Getting Started with Node.js

## Installation

- Using NPM:
```
$ npm i @iota/client
```
- Using yarn:
```
$ yarn add @iota/client
```

## Example

```javascript
const { ClientBuilder } = require('@iota/client')
const client = new ClientBuilder()
    .node('https://api.lb-0.h.chrysalis-devnet.iota.cafe')
    .build()
client.getInfo().then(console.log).catch(console.error)
```
