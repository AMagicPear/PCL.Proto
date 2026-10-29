import { reactive } from 'vue'

/** 顶部导航栏中的一级标签页 */
export const topLevelTabs: string[] = ['/home', '/download', '/setup', '/tools']

/** 一级标签页 -> 上次停留的二级标签页路径 */
const lastSubTab = reactive<Record<string, string>>({})

/** 取路径的一级标签页根路径（/download/game/inner -> /download） */
function rootOf(path: string): string {
  return '/' + (path.split(/[/?#]/).filter(Boolean)[0] ?? '')
}

/** 取回某个一级标签页上次停留的二级标签页，没有记录时返回 undefined */
export function lastSubTabOf(root: string): string | undefined {
  return lastSubTab[root]
}

/**
 * 记录当前停留的二级标签页。
 * 只记录到第二级，例如 /download/game/inner 记为 /download/game，
 * 以保证切回来时落在二级标签页而不是更深的子页面。
 */
export function rememberSubTab(path: string) {
  const root = rootOf(path)
  if (!topLevelTabs.includes(root)) return
  const segments = path.split('/').filter(Boolean)
  if (segments.length < 2) return
  lastSubTab[root] = `/${segments[0]}/${segments[1]}`
}
