import DeviceListPage from "@/pages/device/DeviceListPage.vue";
import DeviceEditPage from "@/pages/device/DeviceEditPage.vue";

export default [
  {
    name: 'device.list',
    path: '/device/list',
    component: DeviceListPage,
  },
  {
    name: 'device.new',
    path: '/device/new',
    component: DeviceEditPage,
  },
  {
    name: 'device.edit',
    path: '/device/:id/edit',
    component: DeviceEditPage,
  },
];
