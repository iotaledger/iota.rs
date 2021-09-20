const { resolve } = require('path');
const { spawnSync } = require('child_process');

// Passing "--prepack 'npm run build:neon'" causes problems on Windows, so this is a workaround

spawnSync('npm', ['run', 'build:neon'], {
    stdio: 'inherit',
    cwd: resolve(__dirname, '../'),
});
