import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Menu.vue'
import Settings from '../views/Settings.vue'
import Saves_Lists from '../views/List_Saves.vue'
import Manage_Backups from '../views/Manage_Backups.vue'
import Do_Backup from '../views/Do_Backup.vue'

const routes = [
  { path: '/', component: Home },
  { path: '/settings', component: Settings },
  {path: '/saves_list', component: Saves_Lists},
  {path: '/manage_backups', component: Manage_Backups},
  {path: '/do_backup', component: Do_Backup},
]

export default createRouter({
  history: createWebHistory(),
  routes,
})