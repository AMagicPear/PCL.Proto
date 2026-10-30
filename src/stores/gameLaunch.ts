import type GameInstance from '@/types/gameInstance'
import type { pluginType } from '../types/gameInfo'
import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export const useSelectedInstance = defineStore('selected-instance', () => {
  const instance_info = ref<GameInstance | null>(null)
  let generation = 0

  const plugins = computed<pluginType[]>(
    () => Object.keys(instance_info.value?.pluginsVersion || {}) as pluginType[],
  )

  async function fetch() {
    const current = ++generation
    const instance = await invoke<GameInstance | null>('get_active_instance')
    if (current === generation) instance_info.value = instance
  }

  function setInstance(instance: GameInstance | null) {
    generation++
    instance_info.value = instance
  }

  return {
    instance_info,
    plugins,
    fetch,
    setInstance,
  }
})
