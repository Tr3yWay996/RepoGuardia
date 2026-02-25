import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Menu.vue'
import Settings from '../views/Settings.vue'
import Saves_Lists from '../views/List_Saves.vue'
import Backups_List from '../views/List_Backups.vue'
import Do_Backup from '../views/Do_Backup.vue'
import Do_Restore from '../views/Do_Restore.vue'
import Do_Delete from '../views/Do_Delete.vue'

const routes = [
  { path: '/', component: Home },
  { path: '/settings', component: Settings },
  {path: '/saves_list', component: Saves_Lists},
  {path: '/backup_list', component: Backups_List},
  {path: '/do_backup', component: Do_Backup},
  {path: '/do_restore', component: Do_Restore},
  {path: '/do_delete', component: Do_Delete},
]

export default createRouter({
  history: createWebHistory(),
  routes,
})