import SideNavLayout from '@/layout/SideNavLayout.vue'
import { pageNavigation } from '@/layout/pageNavigation'
import { defineComponent, h } from 'vue'
import { useI18n } from 'vue-i18n'
export default defineComponent({
  setup() {
    const { locale } = useI18n()
    return () =>
      h(SideNavLayout, {
        sideNavGroups: pageNavigation(
          'PageToolsLeft',
          {
            ItemGameLink: '/tools/lobby',
            ItemTest: '/tools/wonderbox',
          },
          locale.value,
        ),
      })
  },
})
