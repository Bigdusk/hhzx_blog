import { createRouter, createWebHistory } from 'vue-router'
import axios_util from "@/utils/axios_util";
import type {User} from "@/entity";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      component: () => import("@/views/IndexView.vue"),
      meta: {
        requireAuth: false
      },
      children: [
        {
          path: '/',
          component: () => import("@/views/HomeView.vue"),
          meta: {
            requireAuth: false
          }
        },
        {
          path: '/article/:id',
          component: () => import("@/views/ArticleView.vue"),
          meta: {
            requireAuth: false
          }
        },
        {
          path: '/category/:category_name/:id',
          component: () => import("@/views/CategoryView.vue"),
          meta: {
            requireAuth: false
          }
        },
        {
          path: '/login',
          component: () => import("@/views/LoginView.vue"),
          meta: {
            requireAuth: false
          }
        },
      ]
    },
    {
      path: '/admin',
      component: () => import("@/views/AdminView.vue"),
      meta: {
        requireAuth: true
      },
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
      ],

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
//登录认证
router.beforeEach((to, from, next) => {
  const isLogin = localStorage.getItem('authorization') // 判断用户是否已登录
  if (to.meta.requireAuth) { // 判断路由是否需要登录控制
    if (isLogin) { // 如果已经登录，则继续访问
      next()
    } else { // 如果未登录，则跳转到登录页面
      next({
        path: '/login',
        query: { redirect: to.fullPath } // 将当前路由地址作为参数传递给登录页面，以便登录后跳转回来
      })
    }
  } else { // 如果不需要登录控制，则直接访问
    next()
  }
})
export default router
