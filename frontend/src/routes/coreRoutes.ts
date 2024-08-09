import IndexPage from '@/pages/IndexPage.vue';
import NotFoundPage from '@/pages/NotFoundPage.vue';

export default [
  {
    name: 'index',
    path: '/',
    components: {
      default: IndexPage,
    },
    meta: { serverless: true },
  },
  {
    name: 'settings',
    path: '/settings',
    components: {
      default: () => import('@/pages/SettingsPage.vue'),
    },
    meta: { serverless: true },
  },
  {
    path: '/:pathMatch(.*)*',
    components: {
      default: NotFoundPage,
    },
    meta: { serverless: true },
  },
];
