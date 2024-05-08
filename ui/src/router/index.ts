import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      component: () => import("@/views/IndexView.vue")
    },
    {
      path: '/article/:id',
      component: () => import("@/views/ArticleView.vue")
    },
    {
      path: '/category/:category_name/:id',
      component: () => import("@/views/CategoryView.vue")
    },
    {
      path: '/login',
      component: () => import("@/views/LoginView.vue")
    },
    {
      path: '/admin',
      component: () => import("@/views/AdminView.vue"),
      children: [
        {
          path: '/admin',
          component: () => import("@/components/admin/Home.vue")
        },
        {
          path: '/admin/article_creation',
          component: () => import("@/components/admin/ArticleCreation.vue")
        },
        {
          path: '/admin/article_creation/:id',
          component: () => import("@/components/admin/ArticleEditing.vue")
        },
        {
          path: '/admin/article_editing',
          component: () => import("@/components/admin/ArticleEditing.vue")
        },
        {
          path: '/admin/category',
          component: () => import("@/components/admin/Category.vue")
        },
        {
          path: '/admin/comment',
          component: () => import("@/components/admin/Comment.vue")
        },
        {
          path: '/admin/articles',
          component: () => import("@/components/admin/Articles.vue")
        },
        {
          path: '/admin/user',
          component: () => import("@/components/admin/User.vue")
        },
        {
          path: '/admin/permissions',
          component: () => import("@/components/admin/Permissions.vue")
        }
      ]
    }
  ]
})
router.beforeEach((to, from, next) => {
  // 在每次导航之前滚动到页面顶部
  window.scrollTo({
    top: 0,
    behavior: 'smooth' // 可选的，使滚动平滑进行
  });
  next();
});
export default router
