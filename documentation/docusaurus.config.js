const path = require('path');

module.exports = {
  title: 'IOTA Client',
  url: '/',
  baseUrl: '/',
  themes: ['@docusaurus/theme-classic'],
  plugins: [
    [
      '@docusaurus/plugin-content-docs',
      {
        id: 'iota-rs',
        path: path.resolve(__dirname, 'docs'),
        routeBasePath: 'iota.rs',
        sidebarPath: path.resolve(__dirname, 'sidebars.js'),
        editUrl: 'https://github.com/iotaledger/iota.rs/edit/production/',
        remarkPlugins: [require('remark-code-import'), require('remark-import-partial')],
      }
    ],
  ],
  staticDirectories: [path.resolve(__dirname, 'static')],
};
