import { isTauri } from '@tauri-apps/api/core'
import { LogicalPosition } from '@tauri-apps/api/dpi'
import { Menu } from '@tauri-apps/api/menu'
import { getCurrentWindow } from '@tauri-apps/api/window'

export interface NativeMenuItem {
  text: string
  action: () => void
}

/** 在锚点元素下方弹出系统原生菜单；非 Tauri 环境不执行任何操作。 */
export async function popupNativeMenu(anchor: Element, items: NativeMenuItem[]): Promise<void> {
  if (!isTauri()) return
  const rect = anchor.getBoundingClientRect()
  const menu = await Menu.new({ items })
  await menu.popup(new LogicalPosition(rect.left, rect.bottom), getCurrentWindow())
}
