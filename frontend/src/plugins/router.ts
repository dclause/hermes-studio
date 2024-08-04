import {
  createRouter,
  createWebHistory,
  NavigationGuardNext,
  RouteLocationNormalized,
} from 'vue-router';
import { startLoader, stopLoader } from '@/composables/loader';
import DefaultLayout from '@/layouts/DefaultLayout.vue';
import ServerlessLayout from '@/layouts/ServerlessLayout.vue';
import { routes } from '@/routes';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: routes,
});

/** Before each route: start the loader and load the appropriate layout (depending on connection status). */
router.beforeEach(
  (to: RouteLocationNormalized, _: RouteLocationNormalized, next: NavigationGuardNext) => {
    startLoader();

    if (to.meta.serverless || to.meta.layout === 'ServerlessLayout') {
      // Use the serverless Layout.
      to.meta.layoutComponent = ServerlessLayout;
      next();
      return;
    }

    to.meta.layoutComponent = DefaultLayout;
    next();
  },
);

/** After each route load: stop the loader. */
router.afterEach(() => {
  stopLoader();
});

export default router;
