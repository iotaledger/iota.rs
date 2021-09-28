const path = require('path')
const fs = require('fs')

// Add node fetch stuff (https://github.com/seanmonstar/reqwest/issues/910)
const entryFilePathNode = path.join(__dirname, '../node/client_wasm.js')
const entryFileNode = fs.readFileSync(entryFilePathNode).toString()
let changedFileNode = entryFileNode.replace(
    "let imports = {};",
    "const fetch = require(\'node-fetch\')\r\nglobal.Headers = fetch.Headers\r\nglobal.Request = fetch.Request\r\nglobal.Response = fetch.Response\r\nglobal.fetch = fetch\r\n\r\nlet imports = {};"
)
    // workaround for node.js, find out if that's required only with nodejs 14 or what's the issue there
    .replace(
        "getObject(arg0).now()",
        "Date.now()"
    )
fs.writeFileSync(
    entryFilePathNode,
    changedFileNode
)

