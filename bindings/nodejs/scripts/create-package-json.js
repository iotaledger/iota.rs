const { writeFileSync } = require('fs');
const { resolve } = require('path');

const path = resolve(__dirname, '../lib');

writeFileSync(`${path}/package.json`, JSON.stringify({}));
