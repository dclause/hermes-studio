import PostureControlPage from '@/pages/postures/PostureControlPage.vue';
import PostureCreatePage from '@/pages/postures/PostureCreatePage.vue';
import PostureEditPage from '@/pages/postures/PostureEditPage.vue';
import PostureListPage from '@/pages/postures/PostureListPage.vue';

export default [
  {
    name: 'posture.control',
    path: '/posture/control',
    component: PostureControlPage,
  },
  {
    name: 'posture.list',
    path: '/posture/list',
    component: PostureListPage,
  },
  {
    name: 'posture.new',
    path: '/posture/new',
    component: PostureCreatePage,
  },
  {
    name: 'posture.edit',
    path: '/posture/:id/edit',
    component: PostureEditPage,
  },
];
