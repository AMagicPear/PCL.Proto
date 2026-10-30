import { isTauri } from '@tauri-apps/api/core'
import { getCurrentInstance, onUnmounted, reactive, ref, watch, type Ref } from 'vue'
import {
  emptyInstanceLaunchSettings,
  getInstanceLaunchSettings,
  setInstanceLaunchSettings,
  type InstanceLaunchSettings,
} from '@/api/instanceLaunchSettings'

export function useInstanceLaunchSettings(directory?: Ref<string | undefined>) {
  const settings = reactive<InstanceLaunchSettings>(emptyInstanceLaunchSettings())
  const ready = ref(false)
  const error = ref('')
  let generation = 0
  let queue: Promise<unknown> = Promise.resolve()
  const currentPath = () => (directory ? directory.value : undefined)

  async function load() {
    const revision = ++generation
    const path = currentPath()
    ready.value = false
    error.value = ''
    Object.assign(settings, emptyInstanceLaunchSettings())
    if (!isTauri() || !path) return
    try {
      const saved = await getInstanceLaunchSettings(path)
      if (revision !== generation) return
      Object.assign(settings, saved)
      ready.value = true
    } catch (failure) {
      if (revision === generation) error.value = String(failure)
    }
  }

  function save() {
    const path = currentPath()
    if (!ready.value || !path) return
    const revision = generation
    const payload: InstanceLaunchSettings = JSON.parse(JSON.stringify(settings))
    queue = queue
      .then(() => setInstanceLaunchSettings(path, payload))
      .then(() => {
        if (revision === generation) error.value = ''
      })
      .catch((failure) => {
        if (revision === generation) error.value = String(failure)
        throw failure
      })
      .catch(() => {})
  }

  if (directory) watch(directory, () => void load(), { immediate: true })
  else void load()
  if (getCurrentInstance())
    onUnmounted(() => {
      generation++
    })
  return { settings, ready, error, load, save }
}
