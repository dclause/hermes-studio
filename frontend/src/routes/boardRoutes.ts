import BoardEditPage from '@/pages/board/BoardEditPage.vue';
import BoardListPage from '@/pages/board/BoardListPage.vue';
import BoardShowPage from '@/pages/board/BoardShowPage.vue';

export default [
  {
    name: 'board.list',
    path: '/board/list',
    component: BoardListPage,
  },
  {
    name: 'board.new',
    path: '/board/new',
    component: BoardEditPage,
  },
  {
    name: 'board.show',
    path: '/board/:hid',
    component: BoardShowPage,
  },
  {
    name: 'board.edit',
    path: '/board/:hid/edit',
    component: BoardEditPage,
  },
];
