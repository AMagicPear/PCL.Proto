import SideNavLayout from '@/layout/SideNavLayout.vue'
import { pageNavigation } from '@/layout/pageNavigation'
import { defineComponent, h } from 'vue'
export default defineComponent({
  setup() {
    return () =>
      h(SideNavLayout, {
        sideNavGroups: pageNavigation('PageToolsLeft', {
          ItemGameLink: '/tools/lobby',
          ItemTest: '/tools/wonderbox',
        }),
      })
  },
})
