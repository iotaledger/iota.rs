---
description: Getting started with the official IOTA Client Library Node.js binding.
image: /img/logo/libraries.png
keywords:
- Node.js
- dotenv
- install
- npm
- yarn
- security
- how to
---
# Getting Started With Node.js

## Requirements

Before you can get started with the Node.js binding for the iota.rs client library, please make sure that you have 
installed [Node](https://nodejs.org/en/), 
[Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html), and [NPM](https://www.npmjs.com/) 
or [Yarn](https://yarnpkg.com/).

## Installing the Library

### NPM

If you are using NPM, you can install the binding in your project by running the following command:

```bash
npm i @iota/client
```

### Yarn

If you are using Yarn, you can install the binding in your project by running the following command:

```bash
yarn add @iota/client
```

## Using the Library

The following example shows you how to include the library and connect to a devnet node. 

```javascript
const { ClientBuilder } = require('@iota/client')
const client = new ClientBuilder()
    .node('https://api.lb-0.h.chrysalis-devnet.iota.cafe')
    .build()
client.getInfo().then(console.log).catch(console.error)
```
