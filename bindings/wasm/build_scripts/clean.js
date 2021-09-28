const fs = require('fs')
const path = require('path')
const nodefile = path.join(__dirname, '../node/.gitignore')
fs.unlinkSync(nodefile);
const webfile = path.join(__dirname, '../web/.gitignore')
fs.unlinkSync(webfile);
