import type { Component } from 'vue'

export interface INavItem {
  text: string
  icon?: Component
  linkto?: string
  clickCallback?: (payload: PointerEvent) => any
  action?: { title: string; icon: Component; kind: string }
}

export interface INavItemGroup {
  title?: string
  content: INavItem[]
}
