module.exports = {
  title: 'MetaverseVM',
  description: 'Virtual machine for the Metaverse',
  base: '/frontier/',

  head: [
    ['meta', { name: 'theme-color', content: '#3eaf7c' }],
    ['meta', { name: 'apple-mobile-web-app-capable', content: 'yes' }],
    ['meta', { name: 'apple-mobile-web-app-status-bar-style', content: 'black' }]
  ],

  themeConfig: {
    repo: 'https://github.com/mvs-org/new-frontiers',
    editLinks: false,
    docsDir: 'docs',
    editLinkText: '',
    lastUpdated: false,
    nav: [
      { text: 'Explorer', link: 'https://vm-explorer.mvs.org' }
    ],
    sidebar: [
      'evm',
      'rust-setup',
    ]
  },

  plugins: [
    '@vuepress/plugin-back-to-top',
    '@vuepress/plugin-medium-zoom',
  ]
}
