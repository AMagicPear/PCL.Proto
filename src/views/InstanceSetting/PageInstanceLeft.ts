import SideNavLayout from '@/layout/SideNavLayout.vue'
import { pageNavigation } from '@/layout/pageNavigation'
import { defineComponent, h } from 'vue'
export default defineComponent({
  setup() {
    return () =>
      h(SideNavLayout, {
        sideNavGroups: pageNavigation('PageInstanceLeft', {
          ItemOverall: '/instance_setting/overview',
          ItemSetup: '/instance_setting/setting',
          ItemInstall: '/instance_setting/modify',
          ItemExport: '/instance_setting/export',
          ItemWorld: '/instance_setting/save',
          ItemScreenshot: '/instance_setting/screenshot',
          ItemMod: '/instance_setting/mod',
          ItemResourcePack: '/instance_setting/resourcepack',
          ItemShader: '/instance_setting/shader',
          ItemSchematic: '/instance_setting/schematic',
          ItemServer: '/instance_setting/server',
        }),
      })
  },
})
