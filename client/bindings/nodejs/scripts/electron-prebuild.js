const { spawnSync } = require('child_process');
const { resolve } = require('path');
const ELECTRON_VERSIONS = ['13.6.9', '14.2.9', '15.5.7', '16.2.8', '17.4.11', '18.3.6', '19.0.10'];

for (const version of ELECTRON_VERSIONS) {
    // HACK: make electron-build-env aware of the electron version we're targeting
    process.env['CURRENT_ELECTRON_VERSION'] = version;
    // HACK: there are bugs in prebuild that are preventing us from using its API, so we're using the CLI instead
    spawnSync(
        process.platform === 'win32' ? 'yarn.cmd' : 'yarn',
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
