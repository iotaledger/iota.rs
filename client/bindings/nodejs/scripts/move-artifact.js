const { existsSync, mkdirSync, renameSync } = require('fs');
const { resolve } = require('path');

// Prebuild requires that the binary be in `build/Release` as though it was built with node-gyp

const moveArtifact = () => {
    const path = resolve(__dirname, '../build/Release');

    if (!existsSync(path)) {
        mkdirSync(path, { recursive: true });
    }
    renameSync(resolve(__dirname, '../index.node'), resolve(path, 'index.node'));
};

module.exports = moveArtifact;
