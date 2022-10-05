const { resolve } = require('path');
const { spawnSync } = require('child_process');

// Passing "--prepack 'yarn build:neon'" causes problems on Windows, so this is a workaround

spawnSync(process.platform === 'win32' ? 'yarn.cmd' : 'yarn', ['build:neon'], { stdio: 'inherit', cwd: resolve(__dirname, '../') });
