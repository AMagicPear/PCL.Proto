import SideNavLayout from '@/layout/SideNavLayout.vue'
import { pageNavigation, setupRoutes } from '@/layout/pageNavigation'
import { defineComponent, h } from 'vue'
export default defineComponent({
  setup() {
    return () =>
      h(SideNavLayout, {
        sideNavGroups: pageNavigation('PageSetupLeft', {
          ...setupRoutes,
          ItemGameLink: '/setup/link',
        }),
      })
  },
})
