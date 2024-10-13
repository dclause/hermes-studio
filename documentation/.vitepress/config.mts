import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "HermesStudio User Guide",
  description: "An open-source Robot Management Interface with no code",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    siteTitle: 'HermesStudio',
    logo: {
      light: '/logo-light.svg',
      dark: '/logo-dark.svg'
    },
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Getting started', link: '/getting-started' },
      {
        text: 'v0.1',
        items: [
          { text: 'no other versions yet', link: '...'}
        ]
      }
    ],

    sidebar: [
      {
        text: 'Introduction',
        collapsed: false,
        items: [
          { text: 'What is HermesStudio ?', link: '/what-is-HermesStudio' },
          { text: 'Getting started', link: '/getting-started' },
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/dclause/hermes-studio' }
    ],
    search: {
      provider: 'local'
    },
    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2024-present Dominique CLAUSE'
    }
  },
  head: [['link', { rel: 'icon', href: '/favicon.ico' }]],
  cleanUrls: true,
})
