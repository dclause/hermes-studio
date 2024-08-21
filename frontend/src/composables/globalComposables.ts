import { useRoute, useRouter } from 'vue-router';

export enum HardwareMode {
  NONE,
  VIRTUAL,
  REALTIME,
}

export function logError(error: Error) {
  console.error(error);
}

export function useRedirect() {
  const route = useRoute();
  const router = useRouter();

  return {
    redirect: () => {
      const redirection = route.query.destination as string;
      if (redirection) {
        return router.push({ path: redirection });
      }
      return router.push({ name: 'device.list' });
    },
  };
}
