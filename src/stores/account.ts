import getSkinUrl from '@/api/skinGetter'
import { invoke, isTauri } from '@tauri-apps/api/core'
import { defineStore } from 'pinia'
import { ref } from 'vue'

export type ProfileType = 'microsoft' | 'offline' | 'authlib' | 'yggdrasilConnect'

/** IPC view of `minecraft::profile::McProfile`. */
export interface McProfile {
  profileId: string
  userName: string
  uuid: string
  tokenType: string
  profileType: ProfileType
  description: string
  skinHeadId: string
  server?: string | null
  serverName?: string | null
  serverHomepage?: string | null
}

export const useAccountInfo = defineStore('account-info', () => {
  const username = ref<string>()
  const uuid = ref<string>()
  const profile = ref<McProfile | null>(null)
  const skinUrl = ref<string>()

  async function loadSkin() {
    const currentUuid = uuid.value
    if (!currentUuid || !isTauri() || profile.value?.profileType === 'offline') return
    try {
      await getSkinUrl(currentUuid, (url) => (skinUrl.value = url), 'uuid')
    } catch (error) {
      skinUrl.value = undefined
      console.warn('Failed to load account skin:', error)
    }
  }

  async function initialize() {
    try {
      const account = await invoke<McProfile | null>('get_account')
      profile.value = account
      username.value = account?.userName
      uuid.value = account?.uuid
      void loadSkin()
    } catch (error) {
      console.error('Failed to initialize account:', error)
    }
  }

  return {
    username,
    uuid,
    profile,
    skinUrl,
    initialize,
  }
})
