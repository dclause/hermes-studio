import postureRoutes from '@/routes/postureRoutes';
import animationRoutes from './animationRoutes';
import boardRoutes from './boardRoutes';
import coreRoutes from './coreRoutes';
import deviceRoutes from './deviceRoutes';

export const routes = [
  ...coreRoutes,
  ...boardRoutes,
  ...deviceRoutes,
  ...postureRoutes,
  ...animationRoutes,
];
