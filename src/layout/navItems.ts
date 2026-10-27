import IconLaunch from '@/components/icons/header/IconLaunch.vue'
import IconDownload from '@/components/icons/header/IconDownload.vue'
import IconSetup from '@/components/icons/header/IconSetup.vue'
import IconMore from '@/components/icons/header/IconWrench.vue'

const navItems = [
  { to: '/home', icon: IconLaunch, label: '{DynamicResource Main.Tab.Launch}' },
  { to: '/download', icon: IconDownload, label: '{DynamicResource Main.Tab.Download}' },
  // { to: '/link', icon: IconLink, label: 'main_nav.link' },
  { to: '/setup', icon: IconSetup, label: '{DynamicResource Main.Tab.Settings}' },
  { to: '/tools', icon: IconMore, label: '{DynamicResource Main.Tab.Tools}' },
  // { to: '/more', icon: IconMore, label: 'main_nav.more' },
]

export default navItems
