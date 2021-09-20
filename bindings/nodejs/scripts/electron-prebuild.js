const { spawnSync } = require('child_process');
const { resolve } = require('path');
const ELECTRON_VERSIONS = ['12.0.18', '13.2.3', '14.0.0'];

for (const version of ELECTRON_VERSIONS) {
    // HACK: make electron-build-env aware of the electron version we're targeting
    process.env['CURRENT_ELECTRON_VERSION'] = version;
    // HACK: there are bugs in prebuild that are preventing us from using its API, so we're using the CLI instead
    spawnSync(
        'npx',
        [
            'prebuild',
            '-t',
            version,
            '-r',
            'electron',
            '--prepack',
            'scripts/electron-neon-build.js',
            '--strip',
        ],
        { stdio: 'inherit', cwd: resolve(__dirname, '../') },
    );
}
