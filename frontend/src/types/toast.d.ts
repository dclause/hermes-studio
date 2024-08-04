export declare type ToastStatus = 'info' | 'success' | 'warning' | 'error';

export declare interface Toast {
  model: boolean;
  text: string;
  status: ToastStatus;
  id: number;
  timeout?: number;
}
