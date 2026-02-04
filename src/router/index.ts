import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Menu.vue'
import Settings from '../views/Settings.vue'
import Saves_Lists from '../views/List.vue'

const routes = [
  { path: '/', component: Home },
  { path: '/settings', component: Settings },
  {path: '/saves_list', component: Saves_Lists}
]

export default createRouter({
  history: createWebHistory(),
  routes,
})