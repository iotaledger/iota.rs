const { resolve } = require('path');
const { spawnSync } = require('child_process');

// Based on https://github.com/prebuild/prebuild/blob/master/strip.js
// Prebuild requires that the binary is in `build/Release` as though it was built with node-gyp

const binaryPath = resolve(__dirname, '../build/Release/index.node');
const stripArgs = process.platform === 'darwin' ? '-Sx' : '--strip-all';
spawnSync('strip', [stripArgs, binaryPath], { stdio: 'inherit' });
