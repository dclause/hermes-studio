import { RouteLocationRaw } from 'vue-router';

// Define the type that enforces either href or to.
export type NavigationItem = {
  href?: string;
  to?: RouteLocationRaw;
  id: string;
  label: string;
  icon: string;

  [key: string]: any;
};
