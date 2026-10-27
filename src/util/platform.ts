import { isTauri } from '@tauri-apps/api/core'
import { platform } from '@tauri-apps/plugin-os'

const currentPlatform = isTauri() ? platform() : 'web'
export default currentPlatform
