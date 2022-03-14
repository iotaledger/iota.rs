const { existsSync, writeFileSync } = require('fs');
const { resolve } = require('path');

const path = resolve(__dirname, '../build/Release');

if (!existsSync(path)) {
    mkdirSync(path, { recursive: true });
}

writeFileSync(`${path}/package.json`, JSON.stringify({}));
