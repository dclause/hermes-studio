import DeviceCreatePage from '@/pages/devices/DeviceCreatePage.vue';
import DeviceEditPage from '@/pages/devices/DeviceEditPage.vue';
import DeviceListPage from '@/pages/devices/DeviceListPage.vue';

export default [
  {
    name: 'device.list',
    path: '/device/list',
    component: DeviceListPage,
  },
  {
    name: 'device.new',
    path: '/device/new/:bid?',
    component: DeviceCreatePage,
  },
  {
    name: 'device.edit',
    path: '/device/:id/edit',
    component: DeviceEditPage,
  },
];
