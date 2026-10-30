import { isTauri } from '@tauri-apps/api/core'
import { arch, platform } from '@tauri-apps/plugin-os'

const currentPlatform = isTauri() ? platform() : 'web'
export const currentArch = isTauri() ? arch() : ''
export default currentPlatform
