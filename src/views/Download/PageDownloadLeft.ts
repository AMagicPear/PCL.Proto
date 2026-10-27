import SideNavLayout from '@/layout/SideNavLayout.vue'
import { pageNavigation } from '@/layout/pageNavigation'
import { defineComponent, h } from 'vue'
export default defineComponent({
  setup() {
    return () =>
      h(SideNavLayout, {
        sideNavGroups: pageNavigation('PageDownloadLeft', {
          ItemInstall: '/download/game',
          ItemMod: '/download/mod',
          ItemPack: '/download/modpack',
          ItemDataPack: '/download/datapack',
          ItemResourcePack: '/download/resourcepack',
          ItemShader: '/download/shader',
          ItemWorld: '/download/world',
          ItemFavorites: '/download/favorites',
          ItemClient: '/download/client',
          ItemOptiFine: '/download/optifine',
          ItemForge: '/download/forge',
          ItemNeoForge: '/download/neoforge',
          ItemCleanroom: '/download/cleanroom',
          ItemFabric: '/download/fabric',
          ItemLegacyFabric: '/download/legacyfabric',
          ItemLabyMod: '/download/labymod',
          ItemLiteLoader: '/download/liteloader',
        }),
      })
  },
})
