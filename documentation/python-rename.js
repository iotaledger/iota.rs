// pydoc-markdown generates the files with `_` because they're named like this in the library,
// but docusaurus ignored them if they start with `_`, so we rename them with `_` removed.
const { join, extname, basename } = require('path');
const { readdirSync, readFileSync, renameSync, writeFileSync } = require('fs');

let pythonApiReference = 'docs/libraries/python/references/iota_client/';

for (const oldFile of readdirSync(pythonApiReference)) {
    const extension = extname(oldFile);
    const name = basename(oldFile, extension);
    if (name[0] === '_') {
        let newFileName = name.substring(1) + extension;
        renameSync(join(pythonApiReference, oldFile), join(pythonApiReference, newFileName));

        // Rename sidebar_label and title
        let pythonReference = readFileSync(pythonApiReference + newFileName, 'utf8');
        var re = new RegExp(name, "g");
        pythonReference = pythonReference.replace(re, name.substring(1));
        writeFileSync(pythonApiReference + newFileName, pythonReference);
    }
}

// Also update the sidebar
let sidebarPath = 'docs/libraries/python/references/sidebar.json'
let sidebarConfig = readFileSync(sidebarPath);
sidebarConfig = JSON.parse(sidebarConfig);

sidebarConfig.items[0].items = sidebarConfig.items[0].items.map((path) => {
    let match = 'references/iota_client/_';
    if (path.startsWith(match)) {
        path = path.slice(0, match.length - 1) + path.slice(match.length)
    }
    return path
})
// Write new data
writeFileSync(sidebarPath, JSON.stringify(sidebarConfig, null, 2));