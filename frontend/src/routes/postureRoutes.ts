import PostureControlPage from '@/pages/posture/PostureControlPage.vue';
import PostureCreatePage from '@/pages/posture/PostureCreatePage.vue';
import PostureEditPage from '@/pages/posture/PostureEditPage.vue';
import PostureListPage from '@/pages/posture/PostureListPage.vue';

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
