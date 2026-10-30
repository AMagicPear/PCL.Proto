// import getSkinUrl from "@/util/skinGetter";
import { invoke } from '@tauri-apps/api/core'
import { defineStore } from 'pinia'
import { ref } from 'vue'
import SteveSkin from '/default-skin/Steve_(classic_texture)_JE6.png'

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

  async function initialize() {
    try {
      const account = await invoke<McProfile | null>('get_account')
      profile.value = account
      username.value = account?.userName
      uuid.value = account?.uuid
    } catch (error) {
      console.error('Failed to initialize account:', error)
    }
  }

  return {
    username,
    uuid,
    profile,
    initialize,
  }
})
