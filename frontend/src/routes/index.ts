import boardRoutes from './board';
import coreRoutes from './core';

export const routes = [...coreRoutes, ...boardRoutes];
