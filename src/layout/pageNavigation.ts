import { h } from 'vue'
import type { INavItemGroup } from '@/types/naviOptions'
import PSvgIcon from '@/components/widget/PSvgIcon.vue'
import sourceActions from '@/assets/nav-actions.json'
const navigation: Record<
  string,
  { title: string; items: { id: string; text: string; icon: string }[] }[]
> = {
  PageSetupLeft: [
    {
      title: '{DynamicResource Setup.Left.Category.Game}',
      items: [
        {
          id: 'ItemLaunch',
          text: '{DynamicResource Setup.Left.Item.Launch}',
          icon: 'lucide/rocket',
        },
        { id: 'ItemJava', text: '{DynamicResource Setup.Left.Item.Java}', icon: 'lucide/coffee' },
        {
          id: 'ItemGameManage',
          text: '{DynamicResource Setup.Left.Item.GameManage}',
          icon: 'lucide/book-marked',
        },
      ],
    },
    {
      title: '{DynamicResource Setup.Left.Category.Tools}',
      items: [
        {
          id: 'ItemGameLink',
          text: '{DynamicResource Setup.Left.Item.GameLink}',
          icon: 'lucide/bubbles',
        },
      ],
    },
    {
      title: '{DynamicResource Setup.Left.Category.Launcher}',
      items: [
        { id: 'ItemUI', text: '{DynamicResource Setup.Left.Item.Ui}', icon: 'lucide/palette' },
        {
          id: 'ItemLauncherLanguage',
          text: '{DynamicResource Setup.Language.Title}',
          icon: 'lucide/earth',
        },
        {
          id: 'ItemLauncherMisc',
          text: '{DynamicResource Setup.Left.Item.Misc}',
          icon: 'lucide/monitor-cog',
        },
      ],
    },
    {
      title: '{DynamicResource Setup.Left.Category.About}',
      items: [
        { id: 'ItemAbout', text: '{DynamicResource Setup.Left.Item.About}', icon: 'lucide/info' },
        {
          id: 'ItemUpdate',
          text: '{DynamicResource Setup.Left.Item.Update}',
          icon: 'lucide/refresh-cw',
        },
        {
          id: 'ItemFeedback',
          text: '{DynamicResource Setup.Left.Item.Feedback}',
          icon: 'lucide/message-circle',
        },
        {
          id: 'ItemLog',
          text: '{DynamicResource Setup.Left.Item.Log}',
          icon: 'lucide/scroll-text',
        },
      ],
    },
  ],
  PageDownloadLeft: [
    {
      title: '',
      items: [
        {
          id: 'ItemInstall',
          text: '{DynamicResource Download.Left.Minecraft}',
          icon: 'lucide/boxes',
        },
      ],
    },
    {
      title: '{DynamicResource Download.Left.CommunityResources}',
      items: [
        { id: 'ItemMod', text: '{DynamicResource Download.Left.Mod}', icon: 'lucide/puzzle' },
        { id: 'ItemPack', text: '{DynamicResource Download.Left.Modpack}', icon: 'lucide/package' },
        {
          id: 'ItemDataPack',
          text: '{DynamicResource Download.Left.DataPack}',
          icon: 'lucide/file-archive',
        },
        {
          id: 'ItemResourcePack',
          text: '{DynamicResource Download.Left.ResourcePack}',
          icon: 'lucide/layers',
        },
        {
          id: 'ItemShader',
          text: '{DynamicResource Download.Left.Shader}',
          icon: 'lucide/sparkles',
        },
        { id: 'ItemWorld', text: '{DynamicResource Download.Left.World}', icon: 'lucide/globe' },
        {
          id: 'ItemFavorites',
          text: '{DynamicResource Download.Left.Favorites}',
          icon: 'lucide/heart',
        },
      ],
    },
    {
      title: '{DynamicResource Download.Left.Installation}',
      items: [
        {
          id: 'ItemClient',
          text: '{DynamicResource Download.Left.Minecraft}',
          icon: 'lucide/package',
        },
        {
          id: 'ItemOptiFine',
          text: '{DynamicResource Download.Left.Optifine}',
          icon: 'lucide/gauge',
        },
        { id: 'ItemForge', text: '{DynamicResource Download.Left.Forge}', icon: 'lucide/anvil' },
        {
          id: 'ItemNeoForge',
          text: '{DynamicResource Download.Left.NeoForge}',
          icon: 'lucide/cat',
        },
        {
          id: 'ItemCleanroom',
          text: '{DynamicResource Download.Left.Cleanroom}',
          icon: 'lucide/flask-conical',
        },
        { id: 'ItemFabric', text: '{DynamicResource Download.Left.Fabric}', icon: 'lucide/scroll' },
        {
          id: 'ItemLegacyFabric',
          text: '{DynamicResource Download.Left.LegacyFabric}',
          icon: 'lucide/scroll',
        },
        { id: 'ItemLabyMod', text: '{DynamicResource Download.Left.LabyMod}', icon: 'lucide/box' },
        {
          id: 'ItemLiteLoader',
          text: '{DynamicResource Download.Left.LiteLoader}',
          icon: 'lucide/egg',
        },
      ],
    },
  ],
  PageToolsLeft: [
    {
      title: '{DynamicResource Tools.Left.Multiplayer}',
      items: [
        { id: 'ItemGameLink', text: '{DynamicResource Tools.Left.Lobby}', icon: 'lucide/link-2' },
      ],
    },
    {
      title: '{DynamicResource Tools.Left.Utilities}',
      items: [
        {
          id: 'ItemTest',
          text: '{DynamicResource Tools.Left.Utilities.Title}',
          icon: 'lucide/tool-case',
        },
      ],
    },
  ],
  PageInstanceLeft: [
    {
      title: '{DynamicResource Instance.Left.Game}',
      items: [
        {
          id: 'ItemOverall',
          text: '{DynamicResource Instance.Left.Overview}',
          icon: 'lucide/layout-dashboard',
        },
        {
          id: 'ItemSetup',
          text: '{DynamicResource Instance.Left.Settings}',
          icon: 'lucide/settings',
        },
        {
          id: 'ItemInstall',
          text: '{DynamicResource Instance.Left.Install}',
          icon: 'lucide/wrench',
        },
        {
          id: 'ItemExport',
          text: '{DynamicResource Instance.Left.Export}',
          icon: 'lucide/package',
        },
      ],
    },
    {
      title: '{DynamicResource Instance.Left.Resources}',
      items: [
        { id: 'ItemWorld', text: '{DynamicResource Instance.Left.Saves}', icon: 'lucide/globe' },
        {
          id: 'ItemScreenshot',
          text: '{DynamicResource Instance.Left.Screenshots}',
          icon: 'lucide/image',
        },
        { id: 'ItemMod', text: '{DynamicResource Instance.Left.Mod}', icon: 'lucide/puzzle' },
        {
          id: 'ItemModDisabled',
          text: '{DynamicResource Instance.Left.Mod}',
          icon: 'lucide/puzzle',
        },
        {
          id: 'ItemResourcePack',
          text: '{DynamicResource Instance.Left.ResourcePacks}',
          icon: 'lucide/layers',
        },
        {
          id: 'ItemShader',
          text: '{DynamicResource Instance.Left.Shaders}',
          icon: 'lucide/sparkles',
        },
        {
          id: 'ItemSchematic',
          text: '{DynamicResource Instance.Left.Schematics}',
          icon: 'lucide/file-box',
        },
        {
          id: 'ItemServer',
          text: '{DynamicResource Instance.Left.Servers}',
          icon: 'lucide/server',
        },
      ],
    },
  ],
}
import { pclText } from '@/api/pclLocale'

export const setupRoutes: Record<string, string> = {
  ItemLaunch: '/setup/launch',
  ItemJava: '/setup/java',
  ItemGameManage: '/setup/manage',
  ItemUI: '/setup/personalization',
  ItemLauncherLanguage: '/setup/language',
  ItemLauncherMisc: '/setup/misc',
  ItemAbout: '/setup/about',
  ItemUpdate: '/setup/update',
  ItemFeedback: '/setup/feedback',
  ItemLog: '/setup/log',
}
export function pageNavigation(page: string, routes: Record<string, string>): INavItemGroup[] {
  return (navigation[page] || []).map((group) => ({
    title: pclText(group.title),
    content: group.items
      .filter((item) => routes[item.id])
      .map((item) => ({
        text: pclText(item.text),
        linkto: routes[item.id]!,
        icon: () => h(PSvgIcon, { name: item.icon, size: 20 }),
        action: (() => {
          const action = (
            sourceActions as Record<
              string,
              Record<string, { title: string; icon: string; kind: string }>
            >
          )[page]?.[item.id]
          return action
            ? {
                title: pclText(action.title),
                kind: action.kind,
                icon: () => h(PSvgIcon, { name: action.icon, size: 15 }),
              }
            : undefined
        })(),
      })),
  }))
}
