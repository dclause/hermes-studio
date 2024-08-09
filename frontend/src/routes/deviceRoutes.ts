import DeviceCreatePage from '@/pages/devices/DeviceCreatePage.vue';
import NotFoundPage from '@/pages/NotFoundPage.vue';

export default [
  // {
  //   name: 'device.list',
  //   path: '/device/list',
  //   component: BoardListPage,
  // },
  {
    name: 'device.new',
    path: '/device/new/:bid?',
    component: DeviceCreatePage,
  },
  {
    name: 'device.show',
    path: '/device/:bid',
    component: NotFoundPage,
  },
  {
    name: 'device.edit',
    path: '/device/:id/edit',
    component: NotFoundPage,
  },
];
