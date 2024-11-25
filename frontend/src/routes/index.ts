import animationRoutes from './animationRoutes';
import boardRoutes from './boardRoutes';
import coreRoutes from './coreRoutes';
import deviceRoutes from './deviceRoutes';
import expanderRoutes from './expanderRoutes';
import postureRoutes from './postureRoutes';

export const routes = [
  ...coreRoutes,
  ...boardRoutes,
  ...expanderRoutes,
  ...deviceRoutes,
  ...postureRoutes,
  ...animationRoutes,
];
