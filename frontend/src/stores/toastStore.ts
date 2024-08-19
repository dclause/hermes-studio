import type { Toast, ToastStatus } from '@/types/toast';
import { defineStore } from 'pinia';

const defaultTimeout = 2000;

declare interface ToastPayload {
  text: string;
  timeout?: number;
}

export const useToasterStore = defineStore('toaster', {
  state: () => ({
    toasts: [] as Toast[],
  }),
  actions: {
    remove(id: number) {
      this.toasts = this.toasts.filter((toast) => toast.id !== id);
    },
    add(payload: ToastPayload, status: ToastStatus) {
      this.toasts.unshift({
        model: true,
        text: payload.text,
        timeout: payload.timeout ?? defaultTimeout,
        status,
        id: Math.random() * 1000,
      });
      this.toasts = this.toasts.slice(0, 5);
    },

    info(text: string) {
      this.add({ text }, 'info');
    },

    success(text: string) {
      this.add({ text }, 'success');
    },

    warning(text: string) {
      this.add({ text }, 'warning');
    },

    error(text: string) {
      this.add({ text }, 'error');
    },
  },
});
