const path = require('path');
const fs = require('fs');
const { lintAll } = require('./lints');
const generatePackage = require('./utils/generatePackage');

const rustPackageName = "iota_client_wasm";

const RELEASE_FOLDER = path.join(__dirname, '../node/wasm/');
const entryFilePathNode = path.join(RELEASE_FOLDER, rustPackageName + '.js');
const entryFileNode = fs.readFileSync(entryFilePathNode).toString();

// copy TypeScript files into temporary directory.
// replace bindings.ts with local version.

lintAll(entryFileNode);

// Add node-fetch polyfill (https://github.com/seanmonstar/reqwest/issues/910).
let changedFileNode = entryFileNode.replace(
    "let imports = {};",
    `if (!globalThis.fetch) {
    const fetch = require('node-fetch')
    globalThis.Headers = fetch.Headers
    globalThis.Request = fetch.Request
    globalThis.Response = fetch.Response
    globalThis.fetch = fetch
}
let imports = {};`);

fs.writeFileSync(
    entryFilePathNode,
    changedFileNode
);

const newPackage = generatePackage({
    main: 'lib/index.js',
    types: 'lib/index.d.ts',
});

fs.writeFileSync(path.join(RELEASE_FOLDER + "../", 'package.json'), JSON.stringify(newPackage, null, 2));
