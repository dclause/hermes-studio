import {
  createRouter,
  createWebHistory,
  NavigationGuardNext,
  RouteLocationNormalized,
} from 'vue-router';
import { startLoader, stopLoader } from '@/composables/loaderComposables';
import DefaultLayout from '@/layouts/DefaultLayout.vue';
import ServerlessLayout from '@/layouts/ServerlessLayout.vue';
import { routes } from '@/routes';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: routes,
});

/** Before each route: start the loader and load the appropriate layout (depending on connection status). */
router.beforeEach(
  (to: RouteLocationNormalized, from: RouteLocationNormalized, next: NavigationGuardNext) => {
    startLoader();

    // Auto-redirect
    if (!to.query.destination) {
      to.query.destination = from.path;
    }

    if (to.meta.serverless || to.meta.layout === 'ServerlessLayout') {
      // Use the serverless Layout.
      to.meta.layoutComponent = ServerlessLayout;
    } else {
      to.meta.layoutComponent = DefaultLayout;
    }

    next();
    return;
  },
);

/** After each route load: stop the loader. */
router.afterEach(() => {
  stopLoader();
});

export default router;
