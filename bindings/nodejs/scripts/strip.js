const { resolve } = require('path');
const { spawnSync } = require('child_process');

// Based on https://github.com/prebuild/prebuild/blob/master/strip.js

if (process.platform === 'win32') {
    // strip doesn't exist on Windows
    return;
}

const binaryPath = resolve(__dirname, '../build/Release/index.node');
const stripArgs = process.platform === 'darwin' ? '-Sx' : '--strip-all';
spawnSync('strip', [stripArgs, binaryPath], { stdio: 'inherit' });
