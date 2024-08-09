import { ref } from 'vue';

let loadingInterval: number | undefined;
const isLoading = ref<boolean>(false);
const currentProgress = ref<number>(0);
let time = 0;

export const startLoader = () => {
  isLoading.value = true;
  currentProgress.value = 0;
  time = 0;
  loadingInterval = window.setInterval(function () {
    time += 100;
    currentProgress.value = Math.round((1 - Math.exp((-1 * time) / 1000)) * 100);
    if (currentProgress.value >= 100) {
      window.clearInterval(loadingInterval);
    }
  }, 100);
};

export const stopLoader = () => {
  window.clearInterval(loadingInterval);
  isLoading.value = false;
  currentProgress.value = 0;
  time = 0;
};

export const useLoader = () => {
  return {
    isLoading,
    currentProgress,
  };
};
