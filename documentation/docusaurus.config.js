const path = require('path');

module.exports = {
  title: 'IOTA Client',
  url: '/',
  baseUrl: '/',
  themes: ['@docusaurus/theme-classic'],
  themeConfig: {
    navbar: {
        // Workaround to disable broken logo href on test build
        logo: {
            src: 'img/logo/libraries.png.svg',
            href: 'https://wiki.iota.org/',
        },
    },
  },
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
      }
    ],
  ],
  staticDirectories: [path.resolve(__dirname, 'static')],
};
