import { useRoute, useRouter } from 'vue-router';

export enum HardwareMode {
  NONE,
  VIRTUAL,
  REALTIME,
}

export function logError(error: Error) {
  console.error(error);
}

export function redirect() {
  const route = useRoute();
  const redirection = route.query.destination as string;
  if (redirection) {
    return useRouter().push({ path: redirection });
  }
  return useRouter().push({ name: 'device.list' });
}
