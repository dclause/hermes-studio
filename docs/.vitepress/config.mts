import {defineConfig} from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
    title: "Hermes-Studio User Guide",
    description: "An open-source Robot Management Interface with no code",
    base: '/hermes-studio/',
    themeConfig: {
        // https://vitepress.dev/reference/default-theme-config
        siteTitle: 'Hermes-Studio',
        logo: {
            src: '/icons/robot-love-outline.svg',
        },
        nav: [
            {text: 'Home', link: '/'},
            {text: 'Getting started', link: '/getting-started'},
            {text: 'Uer guide', link: '/userguide'},
            {
                text: 'v0.1',
                items: [
                    {text: 'no other versions yet', link: '...'}
                ]
            },
        ],

        sidebar: [
            {
                text: 'Introduction',
                collapsed: false,
                items: [
                    {text: 'What is Hermes-Studio ?', link: '/what-is-Hermes-Studio'},
                    {text: 'Getting started', link: '/getting-started'},
                ]
            },
            {
                text: 'User Guide',
                link: '/userguide',
                collapsed: false,
                items: [
                    {text: 'Configuration Panel', link: '/userguide/configuration-panel'},
                    {text: 'Control Panel', link: '/userguide/control-panel'},
                    {text: 'Timeline Panel', link: '/userguide/timeline-panel'},
                ]
            }
        ],

        socialLinks: [
            {icon: 'github', link: 'https://github.com/dclause/hermes-studio'}
        ],
        search: {
            provider: 'local'
        },
        footer: {
            message: 'Released under the MIT License.',
            copyright: 'Copyright © 2024-present Dominique CLAUSE'
        }
    },
    head: [
        ['link', {rel: "icon", type: "image/png", sizes: "16x16", href: "/hermes-studio/favicons/favicon-16x16.png"}],
        ['link', {rel: "icon", type: "image/png", sizes: "32x32", href: "/hermes-studio/favicons/favicon-32x32.png"}],
    ],
    cleanUrls: true,
})
