const { existsSync, mkdirSync, renameSync } = require('fs');
const { resolve } = require('path');

const path = resolve(__dirname, '../build/Release');

if (!existsSync(path)) {
    mkdirSync(path, { recursive: true });
}
renameSync(resolve(__dirname, '../native/index.node'), resolve(path, 'index.node'));
