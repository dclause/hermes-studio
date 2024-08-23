/// <reference types="vite/client" />
import 'pinia';

declare module '*.vue' {
  import { DefineComponent } from 'vue';
  const component: DefineComponent<object, object, never>;
  export default component;
}
