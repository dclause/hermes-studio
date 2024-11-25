import ExpanderEditPage from '@/pages/extender/ExpanderEditPage.vue';

export default [
  {
    name: 'expander.new',
    path: '/expander/new',
    component: ExpanderEditPage,
  },
  {
    name: 'expander.edit',
    path: '/expander/:id/edit',
    component: ExpanderEditPage,
  },
];
