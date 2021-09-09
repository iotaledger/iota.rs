---
description: Getting started with the official IOTA Client Library Node.js binding.
image: /img/logo/iota_mark_light.png
keywords:
- Node.js
- dotenv
- install
- npm
- yarn
- security
---
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
    .node('https://api.lb-0.testnet.chrysalis2.com')
    .build()
client.getInfo().then(console.log).catch(console.error)
```
