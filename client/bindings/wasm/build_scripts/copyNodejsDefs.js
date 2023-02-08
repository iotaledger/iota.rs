const path = require('path');
const fse = require('fs-extra');

// Copy the TypeScript definitions from the Node.js bindings.
const nodejsBindingsDir = path.join(__dirname, '..', '..', 'nodejs');
const outDir = path.join(__dirname, '..', 'out');
const folders = ['lib', 'types', 'test'];
for (const folder of folders) {
    const sourceDir = path.join(nodejsBindingsDir, folder);
    const destDir = path.join(outDir, folder);
    fse.copySync(sourceDir, destDir, { 'overwrite': true });
}

// Overwrite the Node.js `bindings.ts` file with one which links to Wasm functions instead.
const bindingsSrc = path.join(__dirname, '..', 'lib', 'bindings.ts');
const bindingsDest = path.join(__dirname, '..', 'out', 'lib', 'bindings.ts');
fse.copySync(bindingsSrc, bindingsDest, { 'overwrite': true });
