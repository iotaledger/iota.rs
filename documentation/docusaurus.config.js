const path = require('path');

module.exports = {
  plugins: [
    [
      '@docusaurus/plugin-content-docs',
      {
        id: 'iota-rs',
        path: path.resolve(__dirname, 'docs'),
        routeBasePath: 'iota.rs',
        sidebarPath: path.resolve(__dirname, 'sidebars.js'),
        editUrl: 'https://github.com/iotaledger/iota.rs/edit/production/documentation',
        remarkPlugins: [require('remark-code-import'), require('remark-import-partial')],
        versions: {
          current: {
            label: 'IOTA',
            badge: true
          },
        },
      }
    ],
  ],
  staticDirectories: [path.resolve(__dirname, 'static')],
};
