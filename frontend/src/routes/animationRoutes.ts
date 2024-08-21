import AnimationCreatePage from '@/pages/animation/AnimationCreatePage.vue';
import AnimationEditPage from '@/pages/animation/AnimationEditPage.vue';
import AnimationListPage from '@/pages/animation/AnimationListPage.vue';

export default [
  {
    name: 'animation.list',
    path: '/animation/list',
    component: AnimationListPage,
  },
  {
    name: 'animation.new',
    path: '/animation/new',
    component: AnimationCreatePage,
  },
  {
    name: 'animation.edit',
    path: '/animation/:id/edit',
    component: AnimationEditPage,
  },
];
