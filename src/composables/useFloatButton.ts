import { pclText } from '@/api/pclLocale'
import IconDownload from '@/components/icons/header/IconDownload.vue'
import { shallowReactive, type Component } from 'vue'

export enum FloatButtonType {
  DownloadGame,
  TaskManage,
}

export interface FloatButtonState {
  visible: boolean
  text?: string
  icon?: Component
  title?: string
  type: FloatButtonType
  action?: () => void
}

const floatButtonState = shallowReactive<FloatButtonState>({
  visible: false,
  get text() {
    return this.type === FloatButtonType.DownloadGame
      ? pclText('Download.Install.StartDownload')
      : undefined
  },
  icon: IconDownload,
  get title() {
    return this.type === FloatButtonType.TaskManage ? pclText('Main.Title.TaskManager') : undefined
  },
  type: FloatButtonType.DownloadGame,
})

export const useFloatButton = () => {
  const set = (type: FloatButtonType) => {
    floatButtonState.type = type
  }
  return {
    floatButtonState,
    setFloatButton: set,
  }
}
