import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      component: () => import("@/views/IndexView.vue")
    },
    {
      path: '/article',
      component: () => import("@/views/ArticleView.vue")
    },
    {
      path: '/category',
      component: () => import("@/views/CategoryView.vue")
    },
    {
      path: '/admin',
      component: () => import("@/views/AdminView.vue")
    }
  ]
})

export default router
