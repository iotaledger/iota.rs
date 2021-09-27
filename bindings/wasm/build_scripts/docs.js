const fs = require('fs')
const path = require('path')
const jsdoc2md = require('jsdoc-to-markdown')

const importFile = path.join(__dirname, '../node/client_wasm.js')
const exportFile = path.join(__dirname, '../docs/api-reference.md')
const exportFileDocs = path.join(__dirname, '../../../documentation/docs/libraries/wasm/api_reference.md')

const docsRoot = path.join(__dirname, '../docs')
const docsData = jsdoc2md.renderSync({ files: importFile })

if (!fs.existsSync(docsRoot)) {
  fs.mkdirSync(docsRoot)
}

fs.writeFileSync(exportFile, docsData)
fs.writeFileSync(exportFileDocs, docsData)
