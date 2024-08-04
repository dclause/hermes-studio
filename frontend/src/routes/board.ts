import BoardAddPage from '@/pages/board/BoardAddPage.vue';
import BoardListPage from '@/pages/board/BoardListPage.vue';

export default [
  {
    name: 'board.list',
    path: '/board/list',
    component: BoardListPage,
  },
  {
    name: 'board.new',
    path: '/board/new',
    component: BoardAddPage,
  },
];
