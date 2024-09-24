import { ComputedRef, MaybeRefOrGetter, ref, ShallowRef, toValue, watchEffect } from 'vue';
import { useRoute, useRouter } from 'vue-router';

export enum HardwareMode {
  OFF,
  VIRTUAL,
  REALTIME,
}

export enum CommandMode {
  NONE,
  KEYFRAME,
  COMMAND,
  FULL,
}

export function logError(error: Error) {
  console.error(error);
}

export function useRedirect() {
  const route = useRoute();
  const router = useRouter();

  return {
    redirect: (route_name: string | undefined = undefined, force: boolean = false) => {
      const redirection = route.query.destination as string;
      if (force && route_name) {
        return router.push({ name: route_name });
      } else if (redirection) {
        return router.push({ path: redirection });
      } else if (route_name) {
        return router.push({ name: route_name });
      } else return router.go(-1);
    },
  };
}

export function mapEnumToOptions<T>(
  enumObj: T,
  filters: string[] = [],
): { text: string; value: string }[] {
  return Object.keys(enumObj as object)
    .filter((key) => isNaN(Number(key)) && !filters.includes((enumObj as any)[key])) // Filter out the numeric enum members (reverse mappings)
    .map((key) => ({
      text: (enumObj as any)[key], // Get the value of the enum for that key
      value: key,
    }));
}

export function useFetch(url: MaybeRefOrGetter<string> | ComputedRef<string> | ShallowRef<string>) {
  const data = ref(null);
  const error = ref(null);

  const fetchData = () => {
    // Reset state before fetching..
    data.value = null;
    error.value = null;

    fetch(toValue(url))
      .then((res) => res.json())
      .then((json) => (data.value = json))
      .catch((err) => (error.value = err));
  };

  watchEffect(() => {
    fetchData();
  });

  return { data, error };
}

export function useUploadFile(
  url: MaybeRefOrGetter<string> | ComputedRef<string> | ShallowRef<string>,
  onDone = () => {},
) {
  const error = ref<string>('');
  const progress = ref<number>(0);

  const uploadFile = (file: MaybeRefOrGetter<File> | ComputedRef<File> | ShallowRef<File>) => {
    error.value = '';
    progress.value = 0;
  };

  return { uploadFile, progress, error };
}

export function useFetch2(
  url: MaybeRefOrGetter<string> | ComputedRef<string> | ShallowRef<string>,
) {
  const error = ref<string>('');
  const progress = ref<number>(0);

  const fetch = () => {
    error.value = '';
    progress.value = 0;

    // Create an XMLHttpRequest object
    const xhr = new XMLHttpRequest();

    // On successful completion
    xhr.onload = () => {
      if (xhr.status === 200) {
        progress.value = 0;
        error.value = '';
      } else {
        error.value = `Error: ${xhr.status} ${xhr.statusText}`;
      }
    };

    // Handle errors during upload
    xhr.onprogress = (event) => {
      progress.value = (event.loaded / event.total) * 100;
    };

    xhr.onreadystatechange = () => {
      error.value = `Error ${xhr.status}: ${xhr.statusText}`;
    };

    xhr.onerror = function () {
      error.value = 'An error occurred';
    };

    // Set up the request to your server
    xhr.open('GET', toValue(url));

    // Send request
    xhr.send();
  };

  return { fetch, progress, error };
}
