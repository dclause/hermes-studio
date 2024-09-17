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
    path: '/board/:bid',
    component: BoardShowPage,
  },
  {
    name: 'board.edit',
    path: '/board/:bid/edit',
    component: BoardEditPage,
  },
];
